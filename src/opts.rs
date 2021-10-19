use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "mdmg", about = r#"A scaffold prototype code tool"#)]
pub enum Mdmg {
    #[structopt(about = "Build a scaffold using the template ")]
    Generate {
        #[structopt()]
        template_name: String,

        #[structopt()]
        identify: String,

        #[structopt(short = "d", long = "dry-run")]
        dry_run: bool,
    },
    #[structopt(about = "Show available template lists")]
    List {},
}

pub fn parse_cli_args() -> Mdmg {
    Mdmg::from_args()
}
