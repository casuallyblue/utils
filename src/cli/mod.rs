use clap::{Parser, Subcommand};

use self::repo::RepoCommand;

pub mod repo;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Repo {
        #[command(subcommand)]
        command: RepoCommand,
    },
}
