use std::env::current_dir;
use std::os;
use std::path::Path;
use std::process::Command;

use git2::{Repository, Signature};

use crate::cli::repo::{RepoCommand, RepoType};
use crate::cli::Cli;
use crate::result::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.subcommand {
        crate::cli::Command::Repo { command } => match command {
            RepoCommand::Create { name, repo_type } => match repo_type {
                Some(repo_type) => match repo_type {
                    RepoType::Git => {
                        let repo = if let Some(name) = name {
                            Repository::init(name)
                        } else {
                            Repository::init(".")
                        };

                        match repo {
                            Ok(repo) => {
                                println!("Created git repository {:?}", repo.path());
                            }
                            Err(e) => return Err(Box::new(e)),
                        }
                    }
                    RepoType::Pijul => todo!(),
                    RepoType::Subversion => todo!(),
                    RepoType::Bazaar => todo!(),
                },
                None => todo!(),
            },
            RepoCommand::AddChange { path } => match detect_repo_type() {
                RepoType::Git => {
                    let repo = match Repository::open(current_dir().expect("No current directory"))
                    {
                        Ok(repo) => repo,
                        Err(e) => return Err(Box::new(e)),
                    };

                    let mut index = repo.index().expect("Could not get repository index");
                    index
                        .add_path(&Path::new(&path))
                        .expect("could not add path to index");

                    index.write();
                }
                RepoType::Pijul => todo!(),
                RepoType::Subversion => todo!(),
                RepoType::Bazaar => todo!(),
            },
            RepoCommand::Commit { message } => match detect_repo_type() {
                RepoType::Git => {
                    let repo = match Repository::open(current_dir().expect("No current directory"))
                    {
                        Ok(repo) => repo,
                        Err(e) => return Err(Box::new(e)),
                    };

                    let tree_oid = repo.index().unwrap().write_tree().unwrap();

                    let sig =
                        Signature::now("casually-blue", "darkforestsilence@gmail.com").unwrap();

                    let parent_ref = repo.find_reference("HEAD").unwrap();
                    let parent = repo
                        .find_commit(parent_ref.resolve().unwrap().target().unwrap())
                        .unwrap();

                    repo.commit(
                        Some("HEAD"),
                        &sig,
                        &sig,
                        message.as_str(),
                        &repo.find_tree(tree_oid).unwrap(),
                        &[&parent],
                    )
                    .unwrap();
                }
                RepoType::Pijul => todo!(),
                RepoType::Subversion => todo!(),
                RepoType::Bazaar => todo!(),
            },
            RepoCommand::Update => todo!(),
        },
    }
    Ok(())
}

fn detect_repo_type() -> RepoType {
    RepoType::Git
}
