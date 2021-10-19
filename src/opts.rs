use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "mdmg", about = r#"A scaffold prototype code tool"#)]
pub enum Mdmg {
    Generate {
        #[structopt()]
        plan_name: String,

        #[structopt()]
        component_name: String,
    }
}

pub fn parse_cli_args() -> Mdmg {
    Mdmg::from_args()
}
