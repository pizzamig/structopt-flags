extern crate failure;
extern crate structopt_flags;
#[allow(unused_imports)]
#[macro_use]
extern crate structopt;

use failure::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "config", about = "An example using ConfigFile option")]
struct Opt {
    #[structopt(flatten)]
    config: structopt_flags::ConfigFile,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let _config_file = opt.config.get_filename();
    println!("{}", opt.config);
    Ok(())
}
