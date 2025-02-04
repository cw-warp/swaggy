use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Command Parsing Error: {0}")]
    ClapError(#[from] clap::Error),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("JSON Error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("IDL Error: {0}")]
    IdlError(#[from] crate::idl_loader::error::IdlError),
}
