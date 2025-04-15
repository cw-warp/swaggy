pub mod variant_info;

use std::collections::BTreeMap;

use indexmap::{indexmap, IndexMap};
use openapiv3::{
    Components, Info, MediaType, NumberType, ObjectType, OpenAPI, Operation, Parameter, ParameterData, ParameterSchemaOrContent, PathItem, ReferenceOr, Response, Responses, Schema, SchemaData, SchemaKind, StatusCode, StringType, Type
};
use serde_json::{Map, Value};
use variant_info::{VariantInfo, VariantParameter};

use crate::error::CliError;

pub fn process_idl(idl: &Value) -> Result<OpenAPI, CliError> {
    let mut openapi = OpenAPI {
        info: Info {
            title: idl["contract_name"].as_str().unwrap().to_owned(),
            description: Some("CosmWasm Smart Contract Documentation".to_owned()),
            version: idl["contract_version"].as_str().unwrap().to_owned(),
            ..Default::default()
        },
        paths: Default::default(),
        components: Some(Components {
            schemas: Default::default(),
            responses: Default::default(),
            parameters: Default::default(),
            examples: Default::default(),
            request_bodies: Default::default(),
            headers: Default::default(),
            security_schemes: Default::default(),
            links: Default::default(),
            callbacks: Default::default(),
            extensions: IndexMap::new(),
        }),
        openapi: "3.0.0".to_owned(),
        ..Default::default()
    };

    let messages = vec![&idl["execute"], &idl["query"]];

    for msg in messages {
        process_message(msg, &mut openapi);
    }

    Ok(openapi)
}

fn process_message(schema: &Value, api: &mut OpenAPI) {
    let variants = extract_variants(schema).unwrap();
    variants.iter().for_each(|x| {
        generate_path_item(x, api);
    });
}

fn extract_variants(schema_part: &Value) -> Result<Vec<VariantInfo>, CliError> {
    let mut variants = Vec::new();
    let read_call = if schema_part.get("title").unwrap().as_str().unwrap() == "QueryMsg" {
        true
    } else {
        false
    };
    if let Some(one_of) = schema_part.get("oneOf") {
        if let Some(one_of_array) = one_of.as_array() {
            for variant in one_of_array {
                let description = variant
                    .get("description")
                    .map(|x| x.as_str().unwrap().to_owned())
                    .unwrap_or(String::new());
                if let Some(enum_val) = variant.get("enum") {
                    // If it's a simple enum (like "increment")
                    if let Some(enum_array) = enum_val.as_array() {
                        if let Some(name) = enum_array.get(0).and_then(|v| v.as_str()) {
                            variants.push(VariantInfo {
                                name: name.to_string(),
                                description,
                                parameters: BTreeMap::new(),
                                read_call,
                            });
                        }
                    }
                } else if let Some(properties) = variant.get("properties") {
                    let mut required_parameters = Vec::new();
                    let mut parameters = BTreeMap::new();

                    for (param_name, param_value) in properties.as_object().unwrap() {
                        let properties = param_value["properties"].as_object();
                        if properties.is_none() {
                            variants.push(VariantInfo {
                                name: param_name.clone(),
                                description,
                                parameters: BTreeMap::new(),
                                read_call,
                            });

                            break;
                        }
                        let properties = properties.unwrap();
                        let req_array = param_value["required"].as_array().unwrap();
                        let mut req_values = req_array
                            .iter()
                            .map(|x| x.as_str().unwrap().to_owned())
                            .collect::<Vec<_>>();
                        required_parameters.append(&mut req_values);
                        properties.iter().for_each(|(k, v)| {
                            let param_type = if v.get("type").is_some() {
                                v.get("type").unwrap().as_str()
                            } else {
                                v.get("$ref").unwrap().as_str().unwrap().split("/").last()
                            }
                            .unwrap()
                            .to_owned();
                            parameters.insert(
                                k.to_owned(),
                                VariantParameter {
                                    t: param_type,
                                    required: false,
                                },
                            );
                        });
                        required_parameters.iter().for_each(|x| {
                            parameters
                                .entry(x.to_owned())
                                .and_modify(|p| p.required = true);
                        });
                        variants.push(VariantInfo {
                            name: param_name.clone(),
                            description,
                            parameters,
                            read_call,
                        });
                        break;
                    }
                }
            }
        }
    }

    Ok(variants)
}

fn generate_path_item(variant: &VariantInfo, api: &mut OpenAPI) {
    let path_name = format!("/{}", &variant.name);
    let params = variant
        .parameters
        .iter()
        .map(|x| {
             let param = Parameter::Query {
                parameter_data: resolve_parameter_data(x.0, x.1),
                allow_reserved: false,
                style: openapiv3::QueryStyle::Form,
                allow_empty_value: None,
            };
            ReferenceOr::Item(param)
        })
        .collect::<Vec<_>>();
    let result_obj_type = MediaType {
        schema: Some(ReferenceOr::Item(Schema { schema_data: SchemaData { title: Some("Response from the chain".to_owned()), ..Default::default()  }, schema_kind: SchemaKind::Type(Type::Object(ObjectType { ..Default::default() })) })),
        example: Some(Value::Object(Map::new())),
        ..Default::default()
    };
    let ok_response = Response {
        content: indexmap! {
            "application/json".to_owned() => result_obj_type
        },
        description: "A successful response".to_owned(),
        ..Default::default()
    };
    let op = Operation {
        responses: Responses {
            responses: indexmap! {
                StatusCode::Code(200) => openapiv3::ReferenceOr::Item(ok_response)
            },
            ..Default::default()
        },
        summary: Some(
            variant
                .description
                .split("\n")
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .to_string(),
        ),
        description: Some(variant.description.to_owned()),
        parameters: params,
        ..Default::default()
    };
    let variant_value = serde_json::to_value(&variant).unwrap();
    let extensions = indexmap! {
        "x-variant".to_owned() => variant_value
    };
    let path_item = if variant.read_call {
        PathItem {
            get: Some(op),
            extensions,
            ..Default::default()
        }
    } else {
        PathItem {
            post: Some(op),
            extensions,
            ..Default::default()
        }
    };
    api.paths
        .paths
        .insert(path_name, ReferenceOr::Item(path_item));
}

fn resolve_parameter_data(name: &str, parameter: &VariantParameter) -> ParameterData {
    let format = ParameterSchemaOrContent::Schema(ReferenceOr::Item(Schema {
        schema_kind: resolve_type(&parameter.t),
        schema_data: SchemaData {
            ..Default::default()
        },
    }));

    ParameterData {
        name: name.to_owned(),
        description: None,
        required: parameter.required,
        deprecated: None,
        format,
        example: None,
        examples: IndexMap::new(),
        explode: None,
        extensions: IndexMap::new(),
    }
}

fn resolve_type(t: &str) -> SchemaKind {
    // resolve numbers
    let (l, n) = t.split_at(1);
    if (l == "i" || l == "u") && (n == "8" || n == "16" || n == "32" || n == "64") {
        return SchemaKind::Type(Type::Number(NumberType {
            minimum: if l == "u" { Some(0f64) } else { None },
            ..Default::default()
        }));
    }
    match t {
        _ => SchemaKind::Type(Type::String(StringType {
            ..Default::default()
        })),
    }
}
