mod commands;
mod error;
mod markdown;
mod opts;
mod output;
mod scaffold;
mod scaffold_executor;
mod template;
mod template_repository;

use crate::commands::generate::{GenerateCommand, GenerateCommandImpl};
use crate::error::MdmgError;
use crate::opts::{parse_cli_args, Mdmg};

pub type Result<T> = anyhow::Result<T, MdmgError>;

pub fn run() -> Result<()> {
    match parse_cli_args() {
        Mdmg::Generate {
            plan_name,
            component_name,
            dry_run,
        } => {
            let command = GenerateCommandImpl::new();
            command.run(plan_name, component_name, dry_run)?;
        }
    };
    Ok(())
}
