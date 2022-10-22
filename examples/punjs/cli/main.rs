mod cmd;

use anyhow::Result;
use clap::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(about = "A light-weight runtime for TypeScript/JavaScript")]
enum Command {
    /// Run a TypeScript/JavaScript program.
    Run {
        /// Input file
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Run { input } => {
            cmd::run(input)?;
        }
    }
    Ok(())
}
