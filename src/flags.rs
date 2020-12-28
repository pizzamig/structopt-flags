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

#[derive(StructOpt, Debug, Clone)]
pub struct DryRunFlag {
    /// With this flag, no opration with side effects will be executed
    #[structopt(name = "dryrunflag", long = "dry-run", short = "n", global = true)]
    pub dry_run: bool,
}

impl fmt::Display for DryRunFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.dry_run {
            write!(f, "dry_run: True")
        } else {
            write!(f, "dry_run: False")
        }
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct YesFlag {
    /// With this flag, the answer YES is always assumed
    #[structopt(name = "yesflag", long = "yes", short = "y", global = true)]
    pub yes: bool,
}

impl fmt::Display for YesFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.yes {
            write!(f, "yes: True")
        } else {
            write!(f, "yes: False")
        }
    }
}
