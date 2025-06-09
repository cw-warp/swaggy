pub mod error;
pub mod idl;
pub mod loaded_idl;
use error::IdlError;
use log::warn;
use serde_json::Value;

use crate::idl_loader::loaded_idl::LoadedIdl;

pub fn try_load(dir: &str) -> Result<Vec<LoadedIdl>, IdlError> {
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

fn load_json_schema(schema_dir: &std::path::Path) -> Result<Vec<LoadedIdl>, IdlError> {
    let json_files = std::fs::read_dir(schema_dir)?.filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.extension()? == "json" {
            Some(path)
        } else {
            None
        }
    });

    let mut idls = Vec::new();
    for file_path in json_files {
        match load_single_json_file(&file_path) {
            Ok(idl) => idls.push(idl),
            Err(e) => {
                // Log the error but continue processing other files
                warn!("Warning: Failed to load {}: {}", file_path.display(), e);
            }
        }
    }

    Ok(idls)
}

fn load_single_json_file(file_path: &std::path::Path) -> Result<LoadedIdl, IdlError> {
    let json = std::fs::read_to_string(file_path)?;
    let idl: Value = serde_json::from_str(&json)?;
    // Check if there is a pre-built wasm file in the `artifacts` directory (produced by cw-optimizer)
    // WARNING: This is a convention and may not always be present, depending on cw workspace setup. TODO: Make this configurable.
    let artifacts_path = file_path.parent();
    let wasm_file_path = if artifacts_path.is_some() {
        Some(
            artifacts_path
                .unwrap()
                .join("artifacts")
                .join(file_path.file_stem().unwrap())
                .with_extension("wasm"),
        )
    } else {
        None
    };
    let wasm = match wasm_file_path {
        Some(path) if path.exists() => Some(std::fs::read(path)?),
        _ => None,
    };
    Ok(LoadedIdl { idl, wasm })
}
