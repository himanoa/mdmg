use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "mdmg", about = r#"A scaffold prototype code tool"#)]
pub struct Opts {
    #[structopt()]
    pub plan_name: String,

    #[structopt()]
    pub component_name: String,
}

pub fn parse_cli_args() -> Opts {
    Opts::from_args()
}
