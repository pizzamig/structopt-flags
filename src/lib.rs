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

mod ipv4;
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

pub use std::net::Ipv4Addr;

pub trait Ipv4Address {
    fn get_ipv4_addr(&self) -> Option<Ipv4Addr>;
}

pub use ipv4::HostV4Opt;
pub use logopt::LogLevelNoDef;
pub use logopt::LogLevelNoDefLower;
pub use logopt::LogLevelOpt;
pub use logopt::LogLevelOptLower;
pub use verbose::QuietVerbose;
pub use verbose::SimpleVerbose;
pub use verbose::Verbose;
pub use verbose::VerboseNoDef;
