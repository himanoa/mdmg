use crate::Result;
use std::env::current_dir;
use std::fs::create_dir;
use std::process::{exit};
use yansi::Paint;

pub trait SetupCommand {
    fn run(&self) -> Result<()>;
}

pub struct SetupCommandImpl;

impl SetupCommandImpl {
    pub fn new() -> Self {
        SetupCommandImpl
    }
}

impl SetupCommand for SetupCommandImpl {
    fn run(&self) -> Result<()> {
        let setup_path = current_dir()?.join(".mdmg");
        if setup_path.exists() {
            println!("Already setuped");
            exit(1);
        }
        create_dir(".mdmg")?;
        println!("{}", Paint::green("Setup complete mdmg"));
        Ok(())
    }
}
