use std::path::PathBuf;

use crate::cli::{Cli, Execute};
use crate::result::Result;

pub trait RepoActions {
    fn create(&mut self);
    fn update(&mut self);
    fn add_change(&mut self, path: String);
    fn commit(&mut self, message: String);
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
