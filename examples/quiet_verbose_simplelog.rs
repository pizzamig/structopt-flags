use structopt::StructOpt;
#[cfg(feature = "simplelog")]
use structopt_flags::LogLevel;

#[derive(Debug, StructOpt)]
#[structopt(name = "quiet_verbose", about = "An example using quite_verbose flag")]
struct Opt {
    #[structopt(flatten)]
    quiet_verbose: structopt_flags::QuietVerbose,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    #[cfg(feature = "simplelog")]
    opt.quiet_verbose.set_log_level();
    log::debug!("{}", opt.quiet_verbose);
    Ok(())
}
