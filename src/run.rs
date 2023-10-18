use crate::cli::{Cli, Execute};
use crate::result::Result;

pub fn run(mut cli: Cli) -> Result<()> {
    cli.execute()
}
