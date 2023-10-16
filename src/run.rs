use std::env::current_dir;
use std::path::Path;

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository, Signature};

use crate::cli::repo::{RepoCommand, RepoType};
use crate::cli::Cli;
use crate::result::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.subcommand {
        crate::cli::Command::Repo { command } => match command {
            RepoCommand::Create { name, repo_type } => match repo_type {
                Some(repo_type) => match repo_type {
                    RepoType::Git => {
                        let repo = Repository::init(if let Some(name) = name {
                            name
                        } else {
                            ".".into()
                        });

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

                    index.write().unwrap();
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
            RepoCommand::Update => {
                let repo = Repository::open(current_dir().expect("no cwd")).unwrap();

                let mut options = FetchOptions::new();
                let mut callbacks = RemoteCallbacks::new();
                callbacks.transfer_progress(|progress| {
                    if progress.received_objects() == progress.total_objects() {
                        print!(
                            "Resolving deltas {}/{}\r",
                            progress.indexed_deltas(),
                            progress.total_deltas()
                        );
                    } else {
                        print!(
                            "Received {}/{} objects ({}) in {} bytes\r",
                            progress.received_objects(),
                            progress.total_objects(),
                            progress.indexed_objects(),
                            progress.received_bytes()
                        );
                    }

                    return true;
                });

                callbacks.credentials(|_url, username_from_url, _allowed_types| {
                    Cred::ssh_key(
                        username_from_url.unwrap_or("git"),
                        None,
                        Path::new("/Users/admin/.ssh/id_rsa"),
                        None,
                    )
                });

                options.remote_callbacks(callbacks);

                let mut remote = repo.find_remote("origin").unwrap();

                remote
                    .fetch(&["master"], Some(&mut options), Some("fetch"))
                    .unwrap();
            }
        },
    }
    Ok(())
}

fn detect_repo_type() -> RepoType {
    RepoType::Git
}
