use std::{
    io::Write,
    process::{Command, Stdio},
};

use clap::Subcommand;

use super::Execute;

#[derive(Subcommand, Debug)]
pub enum SetupCommand {
    Rust,
}

impl Execute for SetupCommand {
    fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            SetupCommand::Rust => {
                let code = reqwest::blocking::get("https://sh.rustup.rs")?.text()?;

                let mut sh = Command::new("/bin/sh");

                sh.arg("-s").arg("--");

                sh.arg("--default-toolchain").arg("nightly");
                sh.arg("-y").arg("-q");

                sh.stdin(Stdio::piped());

                let mut child = sh.spawn()?;
                child.stdin.as_mut().unwrap().write_all(code.as_bytes())?;

                println!("Installing Rust Toolchain");
                child.wait()?;
                println!("Toolchain installed, please restart your shell");

                Ok(())
            }
        }
    }
}
