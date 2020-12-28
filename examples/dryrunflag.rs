use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dry_run_flag", about = "An example using dry-run flag")]
struct Opt {
    #[structopt(flatten)]
    dry_run_flag: structopt_flags::DryRunFlag,
}

fn main() {
    let opt = Opt::from_args();
    println!("{}", opt.dry_run_flag);
}
