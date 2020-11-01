use log::LevelFilter;
use structopt::StructOpt;
use structopt_flags::GetWithDefault;
#[cfg(feature = "simplelog")]
use structopt_flags::SetLogWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "log_level_opt_nodef",
    about = "An example using the LogLevelNoDef option"
)]
struct Opt {
    #[structopt(flatten)]
    log_level: structopt_flags::LogLevelNoDef,
}

const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Debug;

fn main() {
    let opt = Opt::from_args();
    #[cfg(feature = "simplelog")]
    opt.log_level.set_with_default(DEFAULT_LOG_LEVEL);
    log::debug!("{}", opt.log_level.get_with_default(DEFAULT_LOG_LEVEL));
}
