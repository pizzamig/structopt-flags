#[allow(unused_imports)]
#[macro_use]
extern crate structopt;
extern crate failure;
extern crate structopt_flags;

use failure::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "hostipv4_param", about = "An example using HostV4Param option")]
struct Opt {
    #[structopt(flatten)]
    hostipv4: structopt_flags::HostV4Param,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let ipv4 = opt.hostipv4.host_addr;
    println!("{}", ipv4);
    Ok(())
}
