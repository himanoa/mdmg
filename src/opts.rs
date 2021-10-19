use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "mdmg", about = r#"A scaffold prototype code tool"#)]
pub enum Mdmg {
    Generate {
        #[structopt()]
        plan_name: String,

        #[structopt()]
        component_name: String,

        #[structopt(short = "d", long = "dry-run")]
        dry_run: bool
    }
}

pub fn parse_cli_args() -> Mdmg {
    Mdmg::from_args()
}
