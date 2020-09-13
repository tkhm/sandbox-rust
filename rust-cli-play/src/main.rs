use crate::opts::{Command, CommandOpt};
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

async fn run_subcommand(opts: Command) -> Result<(), CliError> {
    match opts {
        Command::Status => status().await,
    }
}

#[tokio::main]
async fn main() {
    let opt: CommandOpt = CommandOpt::from_args();

    match run_subcommand(opt.command).await {
        Ok(_) => (),
        Err(error) => {
            println!("{:?}", error);
        }
    }
}
