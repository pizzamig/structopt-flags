extern crate failure;
extern crate structopt_flags;
#[allow(unused_imports)]
#[macro_use]
extern crate structopt;
#[allow(unused_imports)]
#[macro_use]
extern crate log;

use failure::Error;
use log::LevelFilter;
use structopt::StructOpt;
use structopt_flags::GetWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "verbosenodef",
    about = "An example using verbosenodef flag"
)]
struct Opt {
    #[structopt(flatten)]
    verbose: structopt_flags::VerboseNoDef,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let log_filter = opt.verbose.get_with_default(LevelFilter::Off);
    println!("{}", log_filter);
    Ok(())
}
