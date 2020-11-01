use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "quiet_verbose", about = "An example using quite_verbose flag")]
struct Opt {
    #[structopt(flatten)]
    quiet_verbose: structopt_flags::QuietVerbose,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{}", opt.quiet_verbose);
    Ok(())
}
