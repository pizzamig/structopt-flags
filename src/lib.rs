//! `structopt-flags` is a collection of reusable CLI flags and options
//! based on structopt
//!
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

mod logopt;
mod verbose;

pub use log::{Level, LevelFilter};
/// This trait is designed to provide a log level compatible with the Log crates
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

pub use logopt::LogLevelOpt;
pub use logopt::LogLevelOptLower;
pub use verbose::QuietVerbose;
pub use verbose::SimpleVerbose;
pub use verbose::Verbose;
