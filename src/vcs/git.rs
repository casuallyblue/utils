use std::{
    env::current_dir,
    io::{stdout, Write},
    path::{Path, PathBuf},
};

use git2::{
    build::CheckoutBuilder, Branch, Cred, FetchOptions, MergeOptions, PushOptions, RemoteCallbacks,
    Repository, Signature,
};

use crate::result::Result;

use super::{Repo, RepoActions};

pub struct GitRepo {
    pub(crate) path: PathBuf,
}

impl Repo for GitRepo {
    fn path(&mut self) -> PathBuf {
        self.path.clone()
    }
}

impl<T: Repo> RepoActions for T {
    fn create(&mut self) -> Result<()> {
        let repo = Repository::init(self.path())?;

        println!("Created git repository {:?}", repo.path());

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let repo = Repository::open(current_dir()?)?;

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
                stdout().flush().expect("Failed to flush stdout");
            }

            true
        });

        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap_or("git"),
                None,
                Path::new("/home/sierra/.ssh/id_rsa"),
                None,
            )
        });

        options.remote_callbacks(callbacks);

        let mut remote = repo.find_remote("origin")?;

        remote.fetch(&["master"], Some(&mut options), Some("fetch"))?;

        let ref_anotated = repo.resolve_reference_from_short_name("origin/master")?;
        let annotated = repo.reference_to_annotated_commit(&ref_anotated)?;

        let (analysis, preference) = repo.merge_analysis(&[&annotated])?;

        let head_ref = repo.find_reference("HEAD")?;

        if analysis.is_fast_forward() {
            let target_oid = annotated.id();
            let symbolic_head_ref = head_ref.symbolic_target().expect("symbolic reference");

            let _target_ref =
                repo.reference(symbolic_head_ref, target_oid, true, "Fast Forward")?;

            let target = repo.find_object(target_oid, Some(git2::ObjectType::Commit))?;

            repo.checkout_tree(&target, Some(CheckoutBuilder::new().force()))?;
        } else if analysis.is_normal() {
            if preference.is_fastforward_only() {
                panic!("Fast Forward wanted, but a merge is necessary");
            }

            repo.merge(
                &[&annotated],
                Some(MergeOptions::new().diff3_style(true)),
                Some(CheckoutBuilder::new().allow_conflicts(true).force()),
            )?;

            if !repo.index()?.has_conflicts() {
                // Create the merge commit
                let merge_commit = repo.resolve_reference_from_short_name("origin/master")?;
                let sig = Signature::now("casually-blue", "darkforestsilence@gmail.com")?;

                let target = Branch::wrap(merge_commit);
                let target = target
                    .name()?
                    .expect("Cannot merge to unnamed branch currently");

                let parent1 = head_ref.peel_to_commit()?;
                let parent2 = repo.find_commit(annotated.id())?;

                let tree_oid = repo.index().unwrap().write_tree()?;
                let tree = repo.find_tree(tree_oid)?;

                let _commit = repo.commit(
                    head_ref.name(),
                    &sig,
                    &sig,
                    format!("Merge commit {}", target).as_str(),
                    &tree,
                    &[&parent1, &parent2],
                )?;

                repo.cleanup_state()?;
            }
        }

        Ok(())
    }

    fn add_change(&mut self, path: String) -> Result<()> {
        let repo = match Repository::open(self.path()) {
            Ok(repo) => repo,
            Err(e) => panic!("Error: {:?}", e),
        };

        let mut index = repo.index()?;
        index.add_path(Path::new(&path))?;

        index.write()?;

        Ok(())
    }

    fn commit(&mut self, message: String) -> Result<()> {
        let repo = Repository::open(current_dir()?)?;
        let tree_oid = repo.index()?.write_tree()?;

        let sig = Signature::now("casually-blue", "darkforestsilence@gmail.com")?;

        let parent_ref = repo.find_reference("HEAD")?;
        let parent = repo.find_commit(
            parent_ref
                .resolve()?
                .target()
                .expect("Found a symbolic reference where a normal reference was needed"),
        )?;

        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            message.as_str(),
            &repo.find_tree(tree_oid)?,
            &[&parent],
        )?;

        Ok(())
    }

    fn push(&mut self, remote: String) -> Result<()> {
        let repo = Repository::open(current_dir()?)?;
        let mut remote = repo.find_remote(remote.as_str())?;

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap_or("git"),
                None,
                Path::new("/home/sierra/.ssh/id_rsa"),
                None,
            )
        });

        remote.push(
            &["refs/heads/master"],
            Some(&mut PushOptions::new().remote_callbacks(callbacks)),
        )?;

        Ok(())
    }
}
