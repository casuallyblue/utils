use std::process::Command;

use crate::cli::repo::{RepoCommand, RepoType};
use crate::cli::Cli;
use crate::result::Result;

pub fn run(cli: Cli) -> Result<()> {
    println!("{cli:?}");

    match cli.subcommand {
        crate::cli::Command::Repo { command } => match command {
            RepoCommand::Create { name, repo_type } => match repo_type {
                Some(repo_type) => match repo_type {
                    RepoType::Git => {
                        let mut git = Command::new("git");
                        git.arg("init");

                        if let Some(name) = name {
                            git.arg(name);
                        }

                        git.output().expect("Failed to execute git");
                    }
                    RepoType::Pijul => todo!(),
                    RepoType::Subversion => todo!(),
                    RepoType::Bazaar => todo!(),
                },
                None => todo!(),
            },
            RepoCommand::AddChange { path } => match detect_repo_type() {
                RepoType::Git => {
                    let mut git = Command::new("git");
                    git.arg("add");
                    git.arg(path);

                    git.output().expect("Failed to execute git");
                }
                RepoType::Pijul => todo!(),
                RepoType::Subversion => todo!(),
                RepoType::Bazaar => todo!(),
            },
            RepoCommand::Commit { message } => todo!(),
            RepoCommand::Update => todo!(),
        },
    }
    Ok(())
}

fn detect_repo_type() -> RepoType {
    RepoType::Git
}
