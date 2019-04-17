#[allow(unused_imports)]
#[macro_use]
extern crate structopt;
extern crate failure;
extern crate structopt_flags;

use failure::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "force_flag", about = "An example using force flag")]
struct Opt {
    #[structopt(flatten)]
    forceflag: structopt_flags::ForceFlag,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    println!("{}", opt.forceflag);
    Ok(())
}
