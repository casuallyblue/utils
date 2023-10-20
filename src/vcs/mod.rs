pub mod git;

use std::path::PathBuf;

use crate::result::Result;

pub trait RepoActions {
    fn create(&mut self) -> Result<()>;
    fn update(&mut self) -> Result<()>;
    fn add_change(&mut self, path: PathBuf) -> Result<()>;
    fn commit(&mut self, message: String) -> Result<()>;
    fn push(&mut self, remote: String) -> Result<()>;
    fn status(&mut self) -> Result<()>;
}

pub trait Repo {
    fn path(&mut self) -> PathBuf;
}
