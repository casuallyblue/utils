mod cli;
mod result;
mod run;

use result::Result;

use std::process::exit;

use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            println!("{e}");
            exit(-1);
        }
    };

    match run::run(cli) {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{e}");
            exit(-1);
        }
    }
}
