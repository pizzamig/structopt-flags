use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "simple_verbose",
    about = "An example using simple verbose flag"
)]
struct Opt {
    #[structopt(flatten)]
    verbose: structopt_flags::SimpleVerbose,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{}", opt.verbose);
    Ok(())
}
