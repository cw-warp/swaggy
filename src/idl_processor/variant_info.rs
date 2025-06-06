use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantInfo {
    pub contract_tag: String,
    pub name: String,
    pub description: String,
    pub parameters: BTreeMap<String, VariantParameter>,
    pub read_call: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantParameter {
    pub t: String,
    pub required: bool,
}
