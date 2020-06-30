mod options {
    use argh::FromArgs;

    #[derive(FromArgs)]
    /// A simple calculation tool
    pub struct Args {
        #[argh(subcommand)]
        pub subcommand: SubCommands,
    }

    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand)]
    pub enum SubCommands {
        Init(Init),
    }

    /// Initialize the repository in the current directory.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "init")]
    pub struct Init {}
}

use anyhow::Result;
use gitoxide_core as core;

pub fn main() -> Result<()> {
    pub use options::*;
    let cli: Args = argh::from_env();
    match cli.subcommand {
        SubCommands::Init(_) => core::init(),
    }
}
