extern crate failure;
extern crate structopt_flags;
#[allow(unused_imports)]
#[macro_use]
extern crate structopt;

use failure::Error;
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

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let _config_file = opt.config.get_with_default("config.toml");
    println!("{}", opt.config);
    Ok(())
}
