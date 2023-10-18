use std::error::Error;

use clap::{Parser, Subcommand};

use self::repo::RepoCommand;

pub mod repo;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Command,
}

impl Execute for Cli {
    fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        self.subcommand.execute()
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Repo {
        #[command(subcommand)]
        command: RepoCommand,
    },
}

impl Execute for Command {
    fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Repo { command } => command.execute(),
        }
    }
}

pub trait Execute {
    fn execute(&mut self) -> Result<(), Box<dyn Error>>;
}
