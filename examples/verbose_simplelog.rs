use structopt::StructOpt;
#[cfg(feature = "simplelog")]
use structopt_flags::LogLevel;

#[derive(Debug, StructOpt)]
#[structopt(name = "verbose", about = "An example using verbose flag")]
struct Opt {
    #[structopt(flatten)]
    verbose: structopt_flags::Verbose,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    #[cfg(feature = "simplelog")]
    opt.verbose.set_log_level();
    log::trace!("{}", opt.verbose);
    Ok(())
}
