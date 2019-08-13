#[allow(unused_imports)]
#[macro_use]
extern crate structopt;
extern crate failure;
extern crate structopt_flags;

use failure::Error;
use std::net::Ipv4Addr;
use structopt::StructOpt;
use structopt_flags::GetWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(name = "hostipv4", about = "An example using HostV4Opt option")]
struct Opt {
    #[structopt(flatten)]
    hostipv4: structopt_flags::HostV4Opt,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let ipv4 = opt.hostipv4.get_with_default(Ipv4Addr::new(127, 0, 0, 1));
    println!("{}", ipv4);
    Ok(())
}
