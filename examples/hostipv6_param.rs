use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "hostipv6_param", about = "An example using HostV6Param option")]
struct Opt {
    #[structopt(flatten)]
    hostipv6: structopt_flags::HostV6Param,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let ipv6 = opt.hostipv6.host_addr;
    println!("{}", ipv6);
    Ok(())
}
