use std::net::{IpAddr, Ipv6Addr};
use structopt::StructOpt;
use structopt_flags::GetWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(name = "hostip", about = "An example using HostOpt option")]
struct Opt {
    #[structopt(flatten)]
    hostip: structopt_flags::HostOpt,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let ip = opt
        .hostip
        .get_with_default(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
    println!("{}", ip);
    Ok(())
}
