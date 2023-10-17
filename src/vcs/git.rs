use std::{
    env::current_dir,
    io::{stdout, Write},
    path::Path,
};

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository, Signature};

use crate::run::{Repo, RepoActions};

pub struct GitRepo {
    pub(crate) path: String,
}

impl Repo for GitRepo {
    fn path(&mut self) -> String {
        self.path.clone()
    }
}

impl<T: Repo> RepoActions for T {
    fn create(&mut self) {
        let repo = Repository::init(self.path());
        match repo {
            Ok(repo) => {
                println!("Created git repository {:?}", repo.path());
            }
            Err(e) => panic!("Error {:?}", e),
        }
    }

    fn update(&mut self) {
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
                stdout().flush().unwrap();
            } else {
                print!(
                    "Received {}/{} objects ({}) in {} bytes\r",
                    progress.received_objects(),
                    progress.total_objects(),
                    progress.indexed_objects(),
                    progress.received_bytes()
                );
                stdout().flush().unwrap();
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

        let target_branch = repo
            .find_branch("master", git2::BranchType::Local)
            .unwrap()
            .get()
            .target()
            .unwrap();

        let target_commit = repo.find_annotated_commit(target_branch).unwrap();

        repo.checkout_tree(&repo.find_object(target_commit.id(), None).unwrap(), None)
            .unwrap();

        repo.set_head_bytes(target_commit.refname_bytes()).unwrap();
    }

    fn add_change(&mut self, path: String) {
        let repo = match Repository::open(self.path()) {
            Ok(repo) => repo,
            Err(e) => panic!("Error: {:?}", e),
        };

        let mut index = repo.index().expect("Could not get repository index");
        index
            .add_path(&Path::new(&path))
            .expect("could not add path to index");

        index.write().unwrap();
    }

    fn commit(&mut self, message: String) {
        let repo = match Repository::open(current_dir().expect("No current directory")) {
            Ok(repo) => repo,
            Err(e) => panic!("Error: {:?}", e),
        };

        let tree_oid = repo.index().unwrap().write_tree().unwrap();

        let sig = Signature::now("casually-blue", "darkforestsilence@gmail.com").unwrap();

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
}
