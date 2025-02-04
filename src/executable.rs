use std::{env, path::PathBuf};

use log::trace;

use crate::error::CliError;

pub trait Executable {
    fn execute(&self, ctx: &ExecutionContext) -> Result<(), CliError>;
}

pub struct ExecutionContext {
    pub project_root: PathBuf,
}

impl ExecutionContext {
    pub fn try_load() -> Result<Self, CliError> {
        let project_root = env::current_dir()?;
        trace!("Project root: {:?}", project_root);
        Ok(Self { project_root })
    }
}
