pub mod error;
pub mod idl;
use error::IdlError;
use idl::Idl;
use openapiv3::{Info, OpenAPI};

pub fn try_load(dir: &str) -> Result<Idl, IdlError> {
    // Look for `schema` dir in the current directory and its parents
    let mut dir = std::path::PathBuf::from(dir);
    loop {
        let schema = dir.join("schema");
        if schema.exists() {
            return Ok(load_json_schema(&schema)?);
        }
        if !dir.pop() {
            break;
        }
    }
    Err(IdlError::SchemaDirNotFound)
}

fn load_json_schema(schema_dir: &std::path::Path) -> Result<Idl, IdlError> {
    let json_files = std::fs::read_dir(schema_dir)?
        .map(|f| f.unwrap().path())
        .filter(|f| f.extension().unwrap_or_default() == "json")
        .collect::<Vec<_>>();
    let json = std::fs::read_to_string(json_files.first().unwrap())?;
    let idl: Idl = serde_json::from_str(&json)?;
    Ok(idl)
}
