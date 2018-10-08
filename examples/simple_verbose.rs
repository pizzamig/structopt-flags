#[allow(unused_imports)]
extern crate failure;
extern crate structopt_flags;
#[macro_use]
extern crate structopt;

use failure::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "simple_verbose",
    about = "An example using verbose flag"
)]
struct Opt {
    #[structopt(flatten)]
    verbose: structopt_flags::SimpleVerbose,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    println!("{}", opt.verbose);
    Ok(())
}
