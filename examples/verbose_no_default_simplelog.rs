use log::LevelFilter;
use structopt::StructOpt;
use structopt_flags::GetWithDefault;
#[cfg(feature = "simplelog")]
use structopt_flags::SetLogWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(name = "verbosenodef", about = "An example using verbosenodef flag")]
struct Opt {
    #[structopt(flatten)]
    verbose: structopt_flags::VerboseNoDef,
}

const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Off;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    #[cfg(feature = "simplelog")]
    opt.verbose.set_with_default(DEFAULT_LOG_LEVEL);
    log::trace!("{}", opt.verbose.get_with_default(DEFAULT_LOG_LEVEL));
    Ok(())
}
