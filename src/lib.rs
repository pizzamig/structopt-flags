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

mod config;
mod force;
mod ip;
mod logopt;
mod verbose;

#[cfg(feature = "simplelog")]
use simplelog::{Config, TermLogger, TerminalMode};

pub use log::{Level, LevelFilter};
/// This trait is designed to provide a log level filter, compatible with the Log crates, derived
/// from an option or a flag. Options that can induce a log level, can implement this trait
///
/// If the simplelog features is configured, the set_log_level() function will provide an easy way
/// to set the log level to the TermLogger
pub trait LogLevel {
    /// Return the level filter.
    ///
    /// The log level could be None if the log has been switched off
    fn get_level_filter(&self) -> LevelFilter;

    /// Return the log level.
    ///
    /// The log level could be None if the log has been switched off
    fn get_log_level(&self) -> Option<Level> {
        self.get_level_filter().to_level()
    }

    #[cfg(feature = "simplelog")]
    /// This function will set the log level provided by the option/flag
    fn set_log_level(&self) {
        TermLogger::init(
            self.get_level_filter(),
            Config::default(),
            TerminalMode::Mixed,
        )
        .unwrap_or(());
    }
}

/// This trait is designed to provide a rude form of default value for options
/// If an option doesn't have a default value, it will implement this trait
pub trait GetWithDefault {
    type Item;
    /// This function can be used to retrieve the value of the command line option
    /// taking in account the default value used as argument
    fn get_with_default<T: Into<Self::Item>>(&self, default: T) -> Self::Item;
}
/// This trait is designed to provide a way to set the log level, when the option
/// or the flag doesn't have a default level
#[cfg(feature = "simplelog")]
pub trait SetLogWithDefault: GetWithDefault {
    /// This function can be used to set the loglevel
    fn set_with_default(&self, default: LevelFilter);
}

pub use crate::config::ConfigFile;
pub use crate::config::ConfigFileNoDef;
pub use crate::force::ForceFlag;
pub use crate::ip::HostOpt;
pub use crate::ip::HostParam;
pub use crate::ip::HostV4Opt;
pub use crate::ip::HostV4Param;
pub use crate::ip::HostV6Opt;
pub use crate::ip::HostV6Param;
pub use crate::logopt::LogLevelNoDef;
pub use crate::logopt::LogLevelNoDefLower;
pub use crate::logopt::LogLevelOpt;
pub use crate::logopt::LogLevelOptLower;
pub use crate::verbose::QuietVerbose;
pub use crate::verbose::SimpleVerbose;
pub use crate::verbose::Verbose;
pub use crate::verbose::VerboseNoDef;
