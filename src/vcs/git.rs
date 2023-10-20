use std::{
    io::{stdout, Write},
    path::PathBuf,
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
        let repo = Repository::open(self.path())?;

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
            let home = home::home_dir().expect("Could not find home");
            let ssh_path = home.join(".ssh/id_rsa");
            Cred::ssh_key(
                username_from_url.unwrap_or("git"),
                None,
                ssh_path.as_path(),
                None,
            )
        });

        options.remote_callbacks(callbacks);

        let mut remote = repo.find_remote("origin")?;

        // fetch changes on the master branch
        // but don't do anything with them yet
        remote.fetch(&["master"], Some(&mut options), Some("fetch"))?;

        // Get the head of the origin branch that is being updated
        let ref_anotated = repo.resolve_reference_from_short_name("origin/master")?;
        // Annotate the commit so we can reference it later
        let annotated = repo.reference_to_annotated_commit(&ref_anotated)?;

        // Check if we need a full merge or if we can just fast forward the branch
        let (analysis, preference) = repo.merge_analysis(&[&annotated])?;

        let head_ref = repo.find_reference("HEAD")?;

        if analysis.is_fast_forward() {
            // get the actual id for the new head
            let target_oid = annotated.id();

            // Find the head pointer reference
            let symbolic_head_ref = head_ref.symbolic_target().expect("symbolic reference");

            // Force write the new head reference into the HEAD reference
            repo.reference(symbolic_head_ref, target_oid, true, "Fast Forward")?;

            // Find the new head commit
            let target = repo.find_object(target_oid, Some(git2::ObjectType::Commit))?;

            // Checkout the updated head and force changes to be updated in existing files
            repo.checkout_tree(&target, Some(CheckoutBuilder::new().force()))?;
        } else if analysis.is_normal() {
            if preference.is_fastforward_only() {
                panic!("Fast Forward wanted, but a merge is necessary");
            }

            // Attempt a merge
            repo.merge(
                &[&annotated],
                Some(MergeOptions::new().diff3_style(true)),
                Some(CheckoutBuilder::new().allow_conflicts(true).force()),
            )?;

            // If we don't have any conflicts we can proceed
            if !repo.index()?.has_conflicts() {
                // Find the remote branch's head
                let merge_commit = repo.resolve_reference_from_short_name("origin/master")?;

                // Find the name for the remote tracking branch so we can log it
                let target = Branch::wrap(merge_commit);
                let target = target
                    .name()?
                    .expect("Cannot merge to unnamed branch currently");

                // Get the local head commit
                let parent1 = head_ref.peel_to_commit()?;

                // Get the remote branch's head commit
                let parent2 = repo.find_commit(annotated.id())?;

                // Update the repository tree with the new content
                let tree_oid = repo.index().unwrap().write_tree()?;
                let tree = repo.find_tree(tree_oid)?;

                let sig = Signature::now("casually-blue", "darkforestsilence@gmail.com")?;

                // Apply the merge with a new commit
                repo.commit(
                    head_ref.name(),
                    &sig,
                    &sig,
                    format!("Merge commit {}", target).as_str(),
                    &tree,
                    &[&parent1, &parent2],
                )?;

                // Clean the state so git knows the merge is finished
                repo.cleanup_state()?;
            }
        }

        Ok(())
    }

    fn add_change(&mut self, path: PathBuf) -> Result<()> {
        let repo = Repository::open(self.path())?;

        // Get the index for the currently checked
        // out branch and insert the file into it
        let mut index = repo.index()?;
        index.add_path(path.canonicalize()?.strip_prefix(self.path())?)?;

        // Write the index back out to the filesystem
        index.write()?;

        Ok(())
    }

    fn commit(&mut self, message: String) -> Result<()> {
        let repo = Repository::open(self.path())?;

        // Get a reference to the current tree
        let tree_oid = repo.index()?.write_tree()?;

        let sig = Signature::now("casually-blue", "darkforestsilence@gmail.com")?;

        // Find the head pointer and resolve it to an acutal commit
        let parent_ref = repo.find_reference("HEAD")?;
        let parent = repo.find_commit(
            parent_ref
                .resolve()?
                .target()
                .expect("Found a symbolic reference where a normal reference was needed"),
        )?;

        // Create a new commit with the current head as a parent
        // and set it as the new head
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
        let repo = Repository::open(self.path())?;
        let mut remote = repo.find_remote(remote.as_str())?;

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            let home = home::home_dir().expect("Could not find home");
            let ssh_path = home.join(".ssh/id_rsa");
            Cred::ssh_key(
                username_from_url.unwrap_or("git"),
                None,
                ssh_path.as_path(),
                None,
            )
        });

        // Push the master branch
        remote.push(
            &["refs/heads/master"],
            Some(&mut PushOptions::new().remote_callbacks(callbacks)),
        )?;

        Ok(())
    }

    fn status(&mut self) -> Result<()> {
        let repo = Repository::open(self.path())?;

        match repo.head()?.shorthand() {
            Some(branch) => {
                println!("On branch {branch}");
            }
            None => {
                println!("Not currently on branch");
            }
        }

        match Branch::wrap(repo.head()?).upstream() {
            Ok(upstream) => {
                let upstream_ref = upstream.get();

                if upstream_ref.peel_to_commit()?.id() == repo.head()?.peel_to_commit()?.id() {
                    println!(
                        "Branch is up to date with {}",
                        upstream_ref.shorthand().unwrap()
                    )
                } else if repo.graph_descendant_of(
                    repo.head()?.peel_to_commit()?.id(),
                    upstream_ref.peel_to_commit()?.id(),
                )? {
                    println!("Branch is ahead of {}", upstream_ref.shorthand().unwrap())
                } else if repo.graph_descendant_of(
                    upstream_ref.peel_to_commit()?.id(),
                    repo.head()?.peel_to_commit()?.id(),
                )? {
                    println!("{} is ahead of branch", upstream_ref.shorthand().unwrap())
                } else {
                    panic!("Error, branch should be somewhere relative to its upstream");
                }
            }
            Err(_) => println!(
                "No upstream branch configured for {}",
                Branch::wrap(repo.head()?).name()?.unwrap_or("(anonymous)")
            ),
        }

        let statuses = repo.statuses(None)?;

        print_status_unstaged(&repo, &statuses)?;
        print_status_staged(&repo, &statuses)?;

        Ok(())
    }
}

fn print_status_unstaged(_repo: &Repository, statuses: &git2::Statuses<'_>) -> Result<()> {
    if statuses
        .iter()
        .filter(|s| !s.status().is_ignored() && s.index_to_workdir().is_some())
        .count()
        > 0
    {
        println!();
        println!("Changes not staged for commit: ");
    }

    for status in statuses
        .iter()
        .filter(|s| !s.status().is_ignored() && s.index_to_workdir().is_some())
    {
        let index_info = status
            .index_to_workdir()
            .ok_or("Could not get index information")?;
        let old_path = index_info.old_file().path();
        let new_path = index_info.new_file().path();

        let status_message = match status.status() {
            s if s.is_wt_new() => "new file: ",
            s if s.is_wt_modified() => "modified: ",
            s if s.is_wt_deleted() => "deleted:  ",
            s if s.is_wt_renamed() => "renamed:  ",
            s if s.is_wt_typechange() => "typechange:",
            _ => continue,
        };

        let path = match (old_path, new_path) {
            (Some(old), Some(new)) if new != old => {
                format!("{} -> {}", old.display(), new.display())
            }
            (Some(old), _) => format!("{}", old.display()),
            (_, Some(new)) => format!("{}", new.display()),
            _ => continue,
        };

        println!("\t{} {}", status_message, path);
    }
    Ok(())
}

fn print_status_staged(_repo: &Repository, statuses: &git2::Statuses<'_>) -> Result<()> {
    if statuses
        .iter()
        .filter(|s| !s.status().is_ignored() && s.head_to_index().is_some())
        .count()
        > 0
    {
        println!();
        println!("Changes to be commited: ");
    }

    for status in statuses
        .iter()
        .filter(|s| !s.status().is_ignored() && s.head_to_index().is_some())
    {
        if status.status().is_ignored() {
            continue;
        }

        let index_info = status
            .head_to_index()
            .ok_or("Could not get index information")?;
        let old_path = index_info.old_file().path();
        let new_path = index_info.new_file().path();

        let status_message = match status.status() {
            s if s.is_index_new() => "new file: ",
            s if s.is_index_modified() => "modified: ",
            s if s.is_index_deleted() => "deleted:  ",
            s if s.is_index_renamed() => "renamed:  ",
            s if s.is_index_typechange() => "typechange:",
            _ => continue,
        };

        let path = match (old_path, new_path) {
            (Some(old), Some(new)) if new != old => {
                format!("{} -> {}", old.display(), new.display())
            }
            (Some(old), _) => format!("{}", old.display()),
            (_, Some(new)) => format!("{}", new.display()),
            _ => continue,
        };

        println!("\t{} {}", status_message, path);
    }
    Ok(())
}
