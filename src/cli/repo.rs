use std::{ffi::OsStr, str::FromStr};

use clap::Subcommand;

#[derive(Subcommand, Clone, Debug)]
pub enum RepoCommand {
    Create {
        name: Option<String>,

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
