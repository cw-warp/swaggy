use openapiv3::{Components, Info, OpenAPI};
use schemars::schema::RootSchema;

use crate::{error::CliError, idl_loader::idl::Idl};

pub fn process_idl(idl: &Idl) -> Result<OpenAPI, CliError> {
    let openapi = OpenAPI {
        info: Info {
            title: idl.contract_name.clone(),
            description: Some("CosmWasm Smart Contract Documentation".to_owned()),
            version: idl.contract_version.clone(),
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
        }),
        ..Default::default()
    };

    Ok(openapi)
}

fn process_message(schema: &RootSchema, api: &mut OpenAPI) {
    schema.schema.
}