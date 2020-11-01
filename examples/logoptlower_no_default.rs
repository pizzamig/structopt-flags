use log::LevelFilter;
use structopt::StructOpt;
use structopt_flags::GetWithDefault;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "log_level_opt_nodef_lower",
    about = "An example using the LogLevelNoDefLower option"
)]
struct Opt {
    #[structopt(flatten)]
    log_level: structopt_flags::LogLevelNoDefLower,
}

fn main() {
    let opt = Opt::from_args();
    let filter_level = opt.log_level.get_with_default(LevelFilter::Error);
    println!("{}", filter_level);
}
