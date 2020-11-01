use std::net::Ipv4Addr;
use structopt::StructOpt;
use structopt_flags::GetWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(name = "hostipv4", about = "An example using HostV4Opt option")]
struct Opt {
    #[structopt(flatten)]
    hostipv4: structopt_flags::HostV4Opt,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let ipv4 = opt.hostipv4.get_with_default(Ipv4Addr::new(127, 0, 0, 1));
    println!("{}", ipv4);
    Ok(())
}
