use std::{fs, path::PathBuf};

use clap::Args;
use log::{info, trace};

use crate::{
    error::CliError,
    executable::{Executable, ExecutionContext},
};

#[derive(Debug, Args)]
pub struct BuildCmd {
    /// The cosmwasm-schema file to use
    pub schema: PathBuf,
    /// Optional wasm binary file to include in the documentation file
    #[arg(short, long)]
    pub wasm: Option<PathBuf>,
}

impl Executable for BuildCmd {
    async fn execute(&self, ctx: &ExecutionContext) -> Result<(), CliError> {
        let dir_string = self.schema.to_string_lossy().to_string();
        let idl = crate::idl_loader::try_load(&dir_string)?;
        info!("IDL file loaded successfully.");

        let wasm_file = if let Some(w) = &self.wasm {
            info!("Reading the wasm file...");
            std::fs::read(w).ok()
        }
        else {
            trace!("No wasm binary provided - it will not be included.");
            None
        };
        let api = crate::idl_processor::process_idl(&idl, wasm_file.as_ref())?;
        let api = serde_json::to_string_pretty(&api)?;
        fs::write(self.schema.join("swagger.json"), &api)?;
        Ok(())
    }
}
