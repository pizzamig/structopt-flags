use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "verbose", about = "An example using verbose flag")]
struct Opt {
    #[structopt(flatten)]
    verbose: structopt_flags::Verbose,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{}", opt.verbose);
    Ok(())
}
