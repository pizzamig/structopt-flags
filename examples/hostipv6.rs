use std::net::Ipv6Addr;
use structopt::StructOpt;
use structopt_flags::GetWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(name = "hostipv6", about = "An example using HostV6Opt option")]
struct Opt {
    #[structopt(flatten)]
    hostipv6: structopt_flags::HostV6Opt,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let ipv6 = opt
        .hostipv6
        .get_with_default(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    println!("{}", ipv6);
    Ok(())
}
