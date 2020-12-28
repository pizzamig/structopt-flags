use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "yes_flag", about = "An example using yes flag")]
struct Opt {
    #[structopt(flatten)]
    yes_flag: structopt_flags::YesFlag,
}

fn main() {
    let opt = Opt::from_args();
    println!("{}", opt.yes_flag);
}
