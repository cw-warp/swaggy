use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdlError {
    #[error("Can't find schema directory")]
    SchemaDirNotFound,
    #[error("JSON Deserialize error: {0}")]
    JsonDeserializeError(#[from] serde_json::Error),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Path Error: {0}")]
    PathError(#[from] std::path::StripPrefixError),
}
