#[allow(unused_imports)]
#[macro_use]
extern crate structopt;
extern crate failure;
extern crate structopt_flags;

use failure::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "quiet_verbose",
    about = "An example using quite_verbose flag"
)]
struct Opt {
    #[structopt(flatten)]
    quiet_verbose: structopt_flags::QuietVerbose,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    println!("{}", opt.quiet_verbose);
    Ok(())
}
