use std::path::PathBuf;

use crate::cli::{Cli, Execute};
use crate::result::Result;

pub trait RepoActions {
    fn create(&mut self) -> Result<()>;
    fn update(&mut self) -> Result<()>;
    fn add_change(&mut self, path: String) -> Result<()>;
    fn commit(&mut self, message: String) -> Result<()>;
    fn push(&mut self, remote: String) -> Result<()>;
}

pub trait Repo {
    fn path(&mut self) -> PathBuf;
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.subcommand {
        crate::cli::Command::Repo { mut command } => command.execute()?,
    }
    Ok(())
}
