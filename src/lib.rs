mod opts;
mod error;
mod scaffold;
mod output;
mod template;

use crate::error::MdmbError;
use crate::opts::{parse_cli_args};

pub type Result<T> = std::result::Result<T, MdmbError>;

pub fn run() -> Result<()> {
    parse_cli_args();
    Ok(())
}
