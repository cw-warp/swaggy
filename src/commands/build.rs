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
}

impl Executable for BuildCmd {
    async fn execute(&self, _ctx: &ExecutionContext) -> Result<(), CliError> {
        let dir_string = self.schema.to_string_lossy().to_string();
        let idl = crate::idl_loader::try_load(&dir_string)?;
        info!("IDL file loaded successfully.");

        let api = crate::idl_processor::process_idl(&idl)?;
        let api = serde_json::to_string_pretty(&api)?;
        fs::write(self.schema.join("swagger.json"), &api)?;
        Ok(())
    }
}
