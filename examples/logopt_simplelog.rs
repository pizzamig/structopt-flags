use structopt::StructOpt;
#[cfg(feature = "simplelog")]
use structopt_flags::LogLevel;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "log_level_opt",
    about = "An example using the LogLevelOpt option"
)]
struct Opt {
    #[structopt(flatten)]
    log_level: structopt_flags::LogLevelOpt,
}

fn main() {
    let opt = Opt::from_args();
    #[cfg(feature = "simplelog")]
    opt.log_level.set_log_level();
    log::info!("{}", opt.log_level);
}
