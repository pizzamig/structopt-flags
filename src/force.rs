use std::fmt;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct ForceFlag {
    /// Force the operation
    #[structopt(name = "forceflag", long = "force", short = "f", global = true)]
    pub force: bool,
}

impl fmt::Display for ForceFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.force {
            write!(f, "force: True")
        } else {
            write!(f, "force: False")
        }
    }
}
