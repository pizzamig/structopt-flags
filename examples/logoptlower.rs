use structopt::StructOpt;
use structopt_flags::LogLevel;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "log_level_opt_lower",
    about = "An example using the LogLevelOptLower option"
)]
struct Opt {
    #[structopt(flatten)]
    log_level: structopt_flags::LogLevelOptLower,
}

fn main() {
    let opt = Opt::from_args();
    let filter_level = opt.log_level.get_level_filter();
    println!("{}", filter_level);
}
