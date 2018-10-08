#[allow(unused_imports)]
#[macro_use]
extern crate structopt;
extern crate structopt_flags;

use structopt::StructOpt;
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
    let filter_level = opt.log_level.get_level_filter();
    println!("{}", filter_level);
}
