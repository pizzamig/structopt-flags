#[allow(unused_imports)]
#[macro_use]
extern crate structopt;
extern crate failure;
extern crate structopt_flags;
#[macro_use]
extern crate log;

use failure::Error;
use structopt::StructOpt;
#[cfg(feature = "simplelog")]
use structopt_flags::LogLevel;

#[derive(Debug, StructOpt)]
#[structopt(name = "quiet_verbose", about = "An example using quite_verbose flag")]
struct Opt {
    #[structopt(flatten)]
    quiet_verbose: structopt_flags::QuietVerbose,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    #[cfg(feature = "simplelog")]
    opt.quiet_verbose.set_log_level();
    debug!("{}", opt.quiet_verbose);
    Ok(())
}
