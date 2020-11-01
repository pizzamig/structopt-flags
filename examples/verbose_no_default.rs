use log::LevelFilter;
use structopt::StructOpt;
use structopt_flags::GetWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(name = "verbosenodef", about = "An example using verbosenodef flag")]
struct Opt {
    #[structopt(flatten)]
    verbose: structopt_flags::VerboseNoDef,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let log_filter = opt.verbose.get_with_default(LevelFilter::Off);
    println!("{}", log_filter);
    Ok(())
}
