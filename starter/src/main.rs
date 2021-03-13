extern crate clap;
extern crate python_starter;

use clap::Clap;
use std::io;
use std::path::Path;

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Python(Python),
}

#[derive(Clap)]
struct Python {
    #[clap(short, long)]
    target_dir: String,
}

fn main() -> Result<(), io::Error> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Python(t) => {
            let target_dir = Path::new(&t.target_dir);
            python_starter::setup(&target_dir)?;
        }
    }

    Ok(())
}
