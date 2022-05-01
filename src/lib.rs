mod commands;
mod delete_executor;
mod error;
mod logger;
mod markdown;
mod opts;
mod output;
mod rename_executor;
mod scaffold;
mod scaffold_executor;
mod template;
mod template_repository;

use commands::rename::{RenameCommandImpl, RenameCommand};

use crate::commands::delete::{DeleteCommand, DeleteCommandImpl};
use crate::commands::generate::{GenerateCommand, GenerateCommandImpl};
use crate::commands::list::{ListCommand, ListCommandImpl};
use crate::commands::setup::{SetupCommand, SetupCommandImpl};
use crate::error::MdmgError;
use crate::opts::{parse_cli_args, Mdmg};

pub type Result<T> = anyhow::Result<T, MdmgError>;

pub fn run() -> Result<()> {
    match parse_cli_args() {
        Mdmg::Generate {
            template_name,
            identify,
            dry_run,
        } => {
            let command = GenerateCommandImpl::new();
            command.run(template_name, identify, dry_run)?;
        }
        Mdmg::List {} => {
            let command = ListCommandImpl::default();
            command.run()?;
        }
        Mdmg::Setup {} => {
            let command = SetupCommandImpl::new();
            command.run()?;
        }
        Mdmg::Delete {
            template_name,
            identify,
        } => {
            let command = DeleteCommandImpl::new();
            command.run(template_name, identify)?;
        }
        Mdmg::Rename { template_name, identify, replaced_identify } => {
            let command = RenameCommandImpl::new();
            command.run(&template_name, &identify, &replaced_identify)?;
        }
    };
    Ok(())
}
