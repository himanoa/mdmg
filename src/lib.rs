mod error;
mod markdown;
mod opts;
mod output;
mod scaffold;
mod template;

use crate::error::MdmbError;
use crate::opts::parse_cli_args;

pub type Result<T> = std::result::Result<T, MdmbError>;

pub fn run() -> Result<()> {
    parse_cli_args();
    Ok(())
}
