use crate::cli::{Cli, Execute};
use crate::result::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.subcommand {
        crate::cli::Command::Repo { mut command } => command.execute()?,
    }
    Ok(())
}
