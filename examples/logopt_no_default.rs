#[allow(unused_imports)]
#[macro_use]
extern crate structopt;
extern crate log;
extern crate structopt_flags;

use log::LevelFilter;
use structopt::StructOpt;
use structopt_flags::GetWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "log_level_opt_nodef",
    about = "An example using the LogLevelNoDef option"
)]
struct Opt {
    #[structopt(flatten)]
    log_level: structopt_flags::LogLevelNoDef,
}

fn main() {
    let opt = Opt::from_args();
    let filter_level = opt.log_level.get_with_default(LevelFilter::Debug);
    println!("{}", filter_level);
}
