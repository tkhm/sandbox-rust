use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CommandOpt {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
#[structopt(about = "rust-cli-play COMMAND [OPTIONS, ...]")]
pub enum Command {
    Status,
}
