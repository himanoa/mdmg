mod error;
mod markdown;
mod opts;
mod output;
mod scaffold;
mod template;
mod template_repository;

use crate::error::MdmgError;
use crate::opts::parse_cli_args;

pub type Result<T> = anyhow::Result<T, MdmgError>;

pub fn run() -> Result<()> {
    parse_cli_args();
    Ok(())
}
