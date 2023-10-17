use std::env::current_dir;

use crate::cli::repo::{RepoCommand, RepoType};
use crate::cli::Cli;
use crate::result::Result;
use crate::vcs::git::GitRepo;

pub trait RepoActions {
    fn create(&mut self);
    fn update(&mut self);
    fn add_change(&mut self, path: String);
    fn commit(&mut self, message: String);
}

pub trait Repo {
    fn path(&mut self) -> String;
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.subcommand {
        crate::cli::Command::Repo { command } => match command {
            RepoCommand::Create { name, repo_type } => match repo_type {
                Some(repo_type) => match repo_type {
                    RepoType::Git => GitRepo {
                        path: name.unwrap_or(".".into()),
                    }
                    .create(),
                    RepoType::Pijul => todo!(),
                    RepoType::Subversion => todo!(),
                    RepoType::Bazaar => todo!(),
                },
                None => todo!(),
            },
            RepoCommand::AddChange { path } => detect_repo_type().add_change(path),
            RepoCommand::Commit { message } => detect_repo_type().commit(message),
            RepoCommand::Update => detect_repo_type().update(),
        },
    }
    Ok(())
}

fn detect_repo_type() -> impl RepoActions + Repo {
    GitRepo {
        path: current_dir().unwrap().to_string_lossy().into(),
    }
}
