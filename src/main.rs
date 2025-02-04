use cli::Cli;
use std::process::exit;

pub mod cli;
pub mod commands;
pub mod error;
pub mod executable;
pub mod idl_loader;
pub mod processor;

fn main() {
    env_logger::init();
    if let Err(x) = Cli::run_cli() {
        eprintln!("Error! {}", x);
        exit(1);
    }
}
