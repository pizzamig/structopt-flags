use std::fmt;
use structopt::StructOpt;

/// This struct implements the `--force` cli option as a boolean flag
///
/// The short form is '-f'
///
/// ```rust
/// extern crate structopt_flags;
/// #[macro_use]
/// extern crate structopt;
///
/// use structopt::StructOpt;
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "force", about = "An example using force flag")]
/// struct Opt {
///     #[structopt(flatten)]
///     force_flag: structopt_flags::ForceFlag,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     if opt.force_flag.force {
///         println!("Force operation");
///     } else {
///         println!("No forced behavior ");
///     }
/// }
/// ```
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
