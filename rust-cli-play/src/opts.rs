use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    pub sub: SubCommands,
}

#[derive(StructOpt)]
#[structopt(about = "rust-cli-play COMMAND [OPTIONS, ...]")]
pub enum SubCommands {
    #[structopt(about = "Display recent status")]
    Status {
        #[structopt(short, long, help = "Enable extended description output.")]
        verbose: String,
    },
}
