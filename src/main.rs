use cli::Cli;
use std::process::exit;

pub mod cli;
pub mod commands;
pub mod error;
pub mod executable;
pub mod idl_loader;
pub mod idl_processor;
pub mod consts;

#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(x) = Cli::run_cli().await {
        eprintln!("Error! {}", x);
        exit(1);
    }
}
