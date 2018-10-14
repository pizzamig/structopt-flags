//! `structopt-flags` is a collection of reusable CLI flags and options
//! based on structopt
//!
//! Every flag or option is already pre-configured and can be used marking the field as `flatten`
//!
//! Currently, a set of verbosity flags and log level option are provided.
//! Some flags and options are provided in multiple configuration, to leave the choice to the
//! developer which one to use.
//! For instance, there are 4 type for vebosity flag:
//! * Verbose: -v flag, it provides a log level with Error as default level
//! * QuietVerbose: -v flag to increase vebosity, -q to decrease it; default level: Warning
//! * SimpleVebose: -v flag as boolean, no default log level provided
//! * VeboseNoDef: -v flag, to increase verbosity, but without a default value
//! # Example
//! ```rust
//! extern crate structopt_flags;
//! #[macro_use]
//! extern crate structopt;
//!
//! use structopt::StructOpt;
//! use structopt_flags::LogLevel; // to access get_log_level
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "verbose", about = "An example using verbose flag")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     verbose: structopt_flags::Verbose,
//! }
//!
//! fn main() {
//!     let opt = Opt::from_args();
//!     let log_level_filter = opt.verbose.get_level_filter();
//! }
//! ```
extern crate log;
#[macro_use]
extern crate structopt;

mod ip;
mod logopt;
mod verbose;

pub use log::{Level, LevelFilter};
/// This trait is designed to provide a log level compatible with the Log crates
/// Options or flags that can provide a log level, will implement this trait
pub trait LogLevel {
    /// Return the log level.
    ///
    /// The log level could be None if the log has been switched off
    fn get_log_level(&self) -> Option<Level>;

    /// Return the level filter.
    ///
    /// The log level could be None if the log has been switched off
    fn get_level_filter(&self) -> LevelFilter;
}

/// This trait is designed to provide a rude form of default value for options
/// If an option doesn't have a default value, it will implement this trait
pub trait GetWithDefault {
    type Item;

    /// This function can be used to retrieve the value of the command line option
    /// taking in account the default value used as argument
    fn get_with_default(&self, default: Self::Item) -> Self::Item;
}

pub use ip::HostOpt;
pub use ip::HostV4Opt;
pub use ip::HostV6Opt;
pub use logopt::LogLevelNoDef;
pub use logopt::LogLevelNoDefLower;
pub use logopt::LogLevelOpt;
pub use logopt::LogLevelOptLower;
pub use verbose::QuietVerbose;
pub use verbose::SimpleVerbose;
pub use verbose::Verbose;
pub use verbose::VerboseNoDef;
