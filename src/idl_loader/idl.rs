use std::collections::BTreeMap;

use schemars::schema::RootSchema;

#[derive(serde::Deserialize, Debug, Default)]
pub struct Idl {
    pub contract_name: String,
    pub contract_version: String,
    pub idl_version: String,
    pub instantiate: Option<RootSchema>,
    pub execute: Option<RootSchema>,
    pub query: Option<RootSchema>,
    pub migrate: Option<RootSchema>,
    pub sudo: Option<RootSchema>,
    pub responses: Option<BTreeMap<String, RootSchema>>,
}
