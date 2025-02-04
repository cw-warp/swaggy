use clap::{Parser, Subcommand};

use crate::{
    commands::build::BuildCmd,
    error::CliError,
    executable::{Executable, ExecutionContext},
};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Build a swagger.json file from the contract schema
    Build(BuildCmd),
}

impl Cli {
    /// Custom entrypoint for the CLI that handles all commands.
    ///
    /// # Example
    ///
    /// ```rs
    /// pub mod cli;
    /// use cli::Cli;
    ///
    /// fn main() {
    ///     if let Err(x) = Cli::run_cli() {
    ///         exit(1);
    ///     }
    /// }
    /// ```
    pub fn run_cli() -> Result<(), CliError> {
        let parser = Cli::parse();

        // Set up execution context and execute the command.
        let ctx = ExecutionContext::try_load()?;
        match parser.command {
            Command::Build(x) => x.execute(&ctx),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
