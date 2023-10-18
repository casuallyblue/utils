use std::{error::Error, io::stdout};

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

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
    GenerateCompletions {
        shell: Shell,
    },
}

impl Execute for Command {
    fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Repo { command } => command.execute(),
            Command::GenerateCompletions { shell } => {
                clap_complete::generate(
                    *shell,
                    &mut Cli::command_for_update(),
                    "utils",
                    &mut stdout(),
                );
                Ok(())
            }
        }
    }
}

pub trait Execute {
    fn execute(&mut self) -> Result<(), Box<dyn Error>>;
}
