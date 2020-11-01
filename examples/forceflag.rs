use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "force_flag", about = "An example using force flag")]
struct Opt {
    #[structopt(flatten)]
    forceflag: structopt_flags::ForceFlag,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{}", opt.forceflag);
    Ok(())
}
