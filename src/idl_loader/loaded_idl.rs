
pub struct LoadedIdl {
    pub idl: serde_json::Value,
    pub wasm: Option<Vec<u8>>,
}