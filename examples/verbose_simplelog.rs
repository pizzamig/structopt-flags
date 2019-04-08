#[allow(unused_imports)]
#[macro_use]
extern crate structopt;
extern crate failure;
extern crate structopt_flags;
#[cfg(feature = "simplelog")]
#[macro_use]
extern crate log;

use failure::Error;
use structopt::StructOpt;
#[cfg(feature = "simplelog")]
use structopt_flags::LogLevel;

#[derive(Debug, StructOpt)]
#[structopt(name = "verbose", about = "An example using verbose flag")]
struct Opt {
    #[structopt(flatten)]
    verbose: structopt_flags::Verbose,
}

#[cfg(feature = "simplelog")]
fn test_log(opt: &Opt) {
    opt.verbose.set_log_level();
    trace!("{}", opt.verbose);
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    test_log(&opt);
    Ok(())
}
