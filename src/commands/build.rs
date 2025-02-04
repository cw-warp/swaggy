use std::{fs, path::PathBuf};

use clap::Args;
use serde_json::to_string;

use crate::{
    error::CliError,
    executable::{Executable, ExecutionContext},
};

#[derive(Debug, Args)]
pub struct BuildCmd {
    pub schema: PathBuf,
}

impl Executable for BuildCmd {
    fn execute(&self, ctx: &ExecutionContext) -> Result<(), CliError> {
        let dir_string = self.schema.to_string_lossy().to_string();
        let idl = crate::idl_loader::try_load(&dir_string)?;
        let api = crate::processor::process_idl(&idl)?;
        println!("{:?}", &api);
        let api = serde_json::to_string_pretty(&api)?;
        fs::write(self.schema.join("swagger.json"), &api)?;
        Ok(())
    }
}
