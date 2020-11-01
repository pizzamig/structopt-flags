use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "hostipv4_param", about = "An example using HostV4Param option")]
struct Opt {
    #[structopt(flatten)]
    hostipv4: structopt_flags::HostV4Param,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let ipv4 = opt.hostipv4.host_addr;
    println!("{}", ipv4);
    Ok(())
}
