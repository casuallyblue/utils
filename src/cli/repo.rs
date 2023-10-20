use std::{env::current_dir, error::Error, ffi::OsStr, path::PathBuf, str::FromStr};

use clap::{builder::PossibleValue, Subcommand, ValueEnum};

use crate::vcs::{git::GitRepo, Repo, RepoActions};

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
        path: PathBuf,
    },
    Commit {
        message: String,
    },
    Update,
    Push {
        #[arg(default_value = "origin")]
        remote: String,
    },

    Status,
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
            RepoCommand::Push { remote } => guess_repo_type()?.push(remote.clone())?,
            RepoCommand::Status => guess_repo_type()?.status()?,
        }

        Ok(())
    }
}

pub fn guess_repo_type() -> Result<impl RepoActions + Repo, Box<dyn Error>> {
    let path = current_dir()?;
    let mut path = Some(path.as_path());

    while let Some(repopath) = path {
        let git_path = repopath.join(".git");

        if git_path.exists() {
            return Ok(GitRepo {
                path: PathBuf::from(repopath),
            });
        }

        path = repopath.parent();
    }

    Err("Could not find any existing repository in the current directory or its parents".into())
}

#[derive(Clone, Debug)]
pub enum RepoType {
    Git,
    Pijul,
    Subversion,
    Bazaar,
}

impl ValueEnum for RepoType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            RepoType::Git,
            RepoType::Pijul,
            RepoType::Subversion,
            RepoType::Bazaar,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(match self {
            RepoType::Git => "git",
            RepoType::Pijul => "pijul",
            RepoType::Subversion => "svn",
            RepoType::Bazaar => "bzr",
        }))
    }
}

impl From<&OsStr> for RepoType {
    fn from(value: &OsStr) -> Self {
        match value.to_str() {
            Some(s) => match RepoType::from_str(s, true) {
                Ok(val) => val,
                _ => panic!("Invalid RepoType {}", s),
            },
            None => panic!("Invalid String value: {:?}", value),
        }
    }
}
