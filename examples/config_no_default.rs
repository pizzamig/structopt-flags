use structopt::StructOpt;
use structopt_flags::GetWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "confignodef",
    about = "An example using ConfigFileNoDef option"
)]
struct Opt {
    #[structopt(flatten)]
    config: structopt_flags::ConfigFileNoDef,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let _config_file = opt.config.get_with_default("config.toml");
    println!("{}", opt.config);
    Ok(())
}
