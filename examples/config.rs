use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "config", about = "An example using ConfigFile option")]
struct Opt {
    #[structopt(flatten)]
    config: structopt_flags::ConfigFile,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let _config_file = opt.config.get_filename();
    println!("{}", opt.config);
    Ok(())
}
