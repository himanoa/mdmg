mod error;
mod markdown;
mod opts;
mod output;
mod scaffold;
mod template;
mod template_repository;
mod commands;
mod scaffold_executor;

use crate::opts::{parse_cli_args, Mdmg};
use crate::error::MdmgError;
use crate::commands::generate::{GenerateCommand, GenerateCommandImpl};

pub type Result<T> = anyhow::Result<T, MdmgError>;

pub fn run() -> Result<()> {
    match parse_cli_args() {
        Mdmg::Generate { plan_name, component_name } => {
            let command = GenerateCommandImpl::new();
            command.run(plan_name, component_name)?;
        }
    };
    Ok(())
}
