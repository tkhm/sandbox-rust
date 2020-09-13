use crate::opts::{SubCommands, Opt};
use derive_more::From;
use std::io;
use structopt::StructOpt;

mod opts;

#[derive(Debug, From)]
enum CliError {
    IoError(io::Error),
}

async fn status() -> Result<(), CliError> {
    Ok(())
}

async fn run_subcommand(opts: SubCommands) -> Result<(), CliError> {
    match opts {
        SubCommands::Status => status().await,
    }
}

#[tokio::main]
async fn main() {
    let opt: Opt = Opt::from_args();

    match run_subcommand(opt.sub).await {
        Ok(_) => (),
        Err(error) => {
            println!("{:?}", error);
        }
    }
}
