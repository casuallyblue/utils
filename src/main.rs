mod cli;
mod result;
mod vcs;

use clap_main::clap_main;
use result::Result;

use cli::{Cli, Execute};

#[clap_main]
pub fn run(mut cli: Cli) -> Result<()> {
    cli.execute()
}
