use std::{env::current_dir, error::Error, ffi::OsStr, path::PathBuf, str::FromStr};

use clap::Subcommand;

use crate::{
    run::{Repo, RepoActions},
    vcs::git::GitRepo,
};

use super::Execute;

#[derive(Subcommand, Clone, Debug)]
pub enum RepoCommand {
    Create {
        #[arg(default_value = ".")]
        path: PathBuf,

        #[arg(long, default_value = "git")]
        repo_type: Option<RepoType>,
    },
    AddChange {
        path: String,
    },
    Commit {
        message: String,
    },
    Update,
}

impl Execute for RepoCommand {
    fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            RepoCommand::Create { path, repo_type } => {
                match repo_type.clone().unwrap_or(RepoType::Git) {
                    RepoType::Git => {
                        GitRepo { path: path.clone() }.create()?;
                    }
                    RepoType::Pijul => todo!(),
                    RepoType::Subversion => todo!(),
                    RepoType::Bazaar => todo!(),
                }
            }
            RepoCommand::AddChange { path } => guess_repo_type()?.add_change(path.clone())?,
            RepoCommand::Commit { message } => guess_repo_type()?.commit(message.clone())?,
            RepoCommand::Update => guess_repo_type()?.update()?,
        }

        Ok(())
    }
}

pub fn guess_repo_type() -> Result<impl RepoActions + Repo, Box<dyn Error>> {
    let path = current_dir()?;
    let path = path.as_path();

    let git_path = path.join(".git");

    if git_path.exists() {
        return Ok(GitRepo {
            path: PathBuf::from(path),
        });
    }

    Err("Could not find any existing repository in the current directory".into())
}

#[derive(Clone, Debug)]
pub enum RepoType {
    Git,
    Pijul,
    Subversion,
    Bazaar,
}

impl From<&OsStr> for RepoType {
    fn from(value: &OsStr) -> Self {
        match value.to_str() {
            Some(s) => match RepoType::from_str(s) {
                Ok(val) => val,
                _ => panic!("Invalid RepoType {}", s),
            },
            None => panic!("Invalid String value: {:?}", value),
        }
    }
}

impl FromStr for RepoType {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "git" => RepoType::Git,
            "pijul" => RepoType::Pijul,
            "svn" => RepoType::Subversion,
            "bzr" => RepoType::Bazaar,
            _ => {
                panic!("Unknown value for repo type {}", s);
            }
        })
    }
}
