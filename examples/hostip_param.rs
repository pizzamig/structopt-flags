use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "hostip_param", about = "An example using HostParam option")]
struct Opt {
    #[structopt(flatten)]
    hostip: structopt_flags::HostParam,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let ip = opt.hostip.host_addr;
    println!("{}", ip);
    Ok(())
}
