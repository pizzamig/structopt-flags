//! `structopt-flags` is a collection of reusable CLI flags and options
//! based on structopt
//!
//! Every flag or option is already pre-configured and can be used marking the field as `flatten`
//!
//! # Verbosity
//! Currently, a set of verbosity flags, connected to log levels option, are provided.
//! Some flags and options are provided in multiple configuration, to leave the choice to the
//! developer which one to use.
//! For instance, there are 4 type for vebosity flag:
//! * Verbose: -v flag, it provides a log level with Error as default level
//! * QuietVerbose: -v flag to increase vebosity, -q to decrease it; default level: Warning
//! * SimpleVebose: -v flag as boolean, no default log level provided
//! * VeboseNoDef: -v flag, to increase verbosity, but without a default value
//! ### Verbose
//! This struct provides the `--verbose` cli option
//!
//! By default, the log level is set to error.
//! Multiple occurrences, will increase the verbosity level up to 4.
//!
//! ```rust
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
//! let opt = Opt::from_args();
//! let filter_level = opt.verbose.get_level_filter();
//! // set log level
//! ```
//! ### VerboseNoDef
//! This struct provides the `--verbose` cli option, without a predefined default
//!
//! Multiple occurrences, will increase the verbosity level
//!
//! ```rust
//! use log::LevelFilter;
//! use structopt::StructOpt;
//! use structopt_flags::GetWithDefault; // to access get_log_level
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt( name = "verbose_no_def", about = "An example using verbose flag with no predefined default")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     verbose: structopt_flags::VerboseNoDef,
//! }
//!
//! let opt = Opt::from_args();
//! let filter_level = opt.verbose.get_with_default(LevelFilter::Off);
//! // set log level
//! ```
//! ### QuietVerbose
//! This struct implements the `--verbose` and the `--quiet` cli options
//!
//! By default, the log level is set to warning.
//! Multiple occurrences of `-v`, will increase the verbosity level up to 3.
//! The flag `-q` is used to decrease verbosity.
//! Using it twice, will silent the log.
//!
//! ```rust
//! use structopt::StructOpt;
//! use structopt_flags::LogLevel; // to access get_log_level
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "quietverbose", about = "An example using quietverbose flag")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     verbose: structopt_flags::QuietVerbose,
//! }
//!
//! let opt = Opt::from_args();
//! let filter_level = opt.verbose.get_level_filter();
//! // set log level
//! ```
//! ### SimpleVerbose
//! This struct implements the `--verbose` cli option as a boolean flag
//!
//! By default, the log level is set to warning.
//! Multiple occurrences of `-v` are not supported
//!
//! ```rust
//! use structopt::StructOpt;
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "verbose", about = "An example using verbose flag")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     verbose: structopt_flags::SimpleVerbose,
//! }
//!
//! let opt = Opt::from_args();
//! if opt.verbose.verbose {
//!     println!("Verbose output enabled");
//! } else {
//!     println!("No verbose output");
//! }
//! ```
//! # Log levels
//! There are several options available to specify a log level:
//!
//! ### LogLevelOpt
//! This struct provides the `--log` and `-L` cli option
//!
//! By default, the log level is set to info.
//!
//! ```rust
//! use structopt::StructOpt;
//! use structopt_flags::LogLevel; // to access get_log_level
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "log_level_opt", about = "An example using the LogLevelOpt option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     log_level: structopt_flags::LogLevelOpt,
//! }
//!
//! let opt = Opt::from_args();
//! let filter_level = opt.log_level.get_level_filter();
//! // set log level
//! ```
//! ### LogLevelOptLower
//! This struct provides the `--log` and `-l` cli option, with a lower case letter
//!
//! By default, the log level is set to info.
//!
//! ```rust
//! use structopt::StructOpt;
//! use structopt_flags::LogLevel; // to access get_log_level
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "log_level_opt_lower", about = "An example using the LogLevelOptLower option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     log_level: structopt_flags::LogLevelOptLower,
//! }
//!
//! let opt = Opt::from_args();
//! let filter_level = opt.log_level.get_level_filter();
//! // set log level
//! ```
//! ### LogLevelNoDev
//! This struct provides the `--log` and `-L` cli option, with no default value
//!
//! No default value is provided
//!
//! ```rust
//! use log::LevelFilter;
//! use structopt::StructOpt;
//! use structopt_flags::GetWithDefault; // to access get_log_level
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "log_level_no_def", about = "An example using the LogLevelNoDef option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     log_level: structopt_flags::LogLevelNoDef,
//! }
//!
//! let opt = Opt::from_args();
//! let filter_level = opt.log_level.get_with_default(LevelFilter::Warn);
//! // set log level
//! ```
//! ### LogLevelNoDefLower
//! This struct provides the `--log` and `-l` cli option, with no default value and with a lower case
//! letter
//!
//! No default value is provided
//!
//! ```rust
//! use log::LevelFilter;
//! use structopt::StructOpt;
//! use structopt_flags::GetWithDefault; // to access get_log_level
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "log_level_no_def_lower", about = "An example using the LogLevelNoDefLower option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     log_level: structopt_flags::LogLevelNoDefLower,
//! }
//!
//! let opt = Opt::from_args();
//! let filter_level = opt.log_level.get_with_default(LevelFilter::Warn);
//! // set log level
//! ```
//!# Configuration file
//!This reusable flag can be used to specify a configuration file
//!
//!### ConfigFile
//!This struct provides the `--config_file` cli option
//! The option is mandatory and require a filename
//!
//! ```should_panic
//! use structopt::StructOpt;
//! use structopt_flags::ConfigFile; // to access get_log_level
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "config-file", about = "An example using config_file option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     config: structopt_flags::ConfigFile,
//! }
//!
//! let opt = Opt::from_args();
//! let config_file = opt.config.get_filename();
//! // use the config file
//! ```
//!
//! ### ConfigFileNoDef
//! This struct provides the `--config_file` cli option
//!
//! The option is no mandatory, but a default value can be provided wit the get_with_default()
//!
//! ```rust
//! use structopt::StructOpt;
//! use structopt_flags::GetWithDefault; // to access get_log_level
//! use std::path::PathBuf;
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "config-file", about = "An example using config_file option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     config: structopt_flags::ConfigFileNoDef,
//! }
//!
//! let opt = Opt::from_args();
//! let config_file = opt.config.get_with_default("config-file.toml");
//! // use the config file
//! ```
//! # Host IP address
//! There several ways to provide an IP address
//!
//! ### HostV4Opt
//! This struct provides the `--host` and `-H` cli option to get an IPv4 address
//!
//! No default is provided
//!
//! ```rust
//! use std::net::Ipv4Addr;
//! use structopt::StructOpt;
//! use structopt_flags::GetWithDefault; // to access get_with_default
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "ipv4_opt", about = "An example using the HostV4Opt option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     host_ip: structopt_flags::HostV4Opt,
//! }
//!
//! let opt = Opt::from_args();
//! let ipv4 = opt.host_ip.get_with_default(Ipv4Addr::new(127,0,0,1));
//! ```
//! ### HostV4Param
//! This struct provides the `--host` and `-H` cli option to get an IPv4 address
//!
//! No default is provided and the parameter is mandatory
//! This option is not global
//!
//! ```should_panic
//! use std::net::Ipv4Addr;
//! use structopt::StructOpt;
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "ipv4_param", about = "An example using the HostV4Param option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     host_ip: structopt_flags::HostV4Param,
//! }
//!
//! let opt = Opt::from_args();
//! let ipv4 = opt.host_ip.host_addr;
//! ```
//! ### HostV6Opt
//! This struct provides the `--host` and `-H` cli option to get an IPv6 address
//!
//! No default is provided
//!
//! ```rust
//! use std::net::Ipv6Addr;
//! use structopt::StructOpt;
//! use structopt_flags::GetWithDefault; // to access get_with_default
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "ipv6_opt", about = "An example using the HostV6Opt option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     host_ip: structopt_flags::HostV6Opt,
//! }
//!
//! let opt = Opt::from_args();
//! let ipv6 = opt.host_ip.get_with_default(Ipv6Addr::new(0,0,0,0,0,0,0,1));
//! ```
//! ### HostV6Param
//! This struct provides the `--host` and `-H` cli option to get an IPv6 address
//!
//! No default is provided and the parameter is mandatory
//! This option is not global
//!
//! ```should_panic
//! use std::net::Ipv6Addr;
//! use structopt::StructOpt;
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "ipv6_param", about = "An example using the HostV6Param option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     host_ip: structopt_flags::HostV6Param,
//! }
//!
//! let opt = Opt::from_args();
//! let ipv6 = opt.host_ip.host_addr;
//! ```
//! ### HostOpt
//! This struct provides the `--host` and `-H` cli option to get ageneric IP address
//!
//! No default is provided
//!
//! ```rust
//! use std::net::{IpAddr,Ipv6Addr};
//! use structopt::StructOpt;
//! use structopt_flags::GetWithDefault; // to access get_with_default
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "ip_opt", about = "An example using the HostOpt option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     host_ip: structopt_flags::HostOpt,
//! }
//!
//! let opt = Opt::from_args();
//! let ip = opt.host_ip.get_with_default(IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1)));
//! ```
//! ### HostParam
//! This struct provides the `--host` and `-H` cli option to get ageneric IP address
//!
//! No default is provided and the parameter is mandatory
//! This option is not global
//!
//! ```should_panic
//! use std::net::{IpAddr,Ipv6Addr};
//! use structopt::StructOpt;
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "ip_param", about = "An example using the HostParam option")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     host_ip: structopt_flags::HostParam,
//! }
//!
//! let opt = Opt::from_args();
//! let ip = opt.host_ip.host_addr;
//! ```
//! # Force flag
//! This struct implements the `--force` cli option as a boolean flag
//!
//! The short form is `-f`
//!
//! ```rust
//! use structopt::StructOpt;
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "force", about = "An example using force flag")]
//! struct Opt {
//!     #[structopt(flatten)]
//!     force_flag: structopt_flags::ForceFlag,
//! }
//!
//! let opt = Opt::from_args();
//! if opt.force_flag.force {
//!     println!("Force operation");
//! } else {
//!     println!("No forced behavior ");
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
/// If the simplelog features is configured, the `set_log_level()` function will provide an easy way
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
    /// if the feature `simplelog` is enabled
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
#[cfg(feature = "simplelog")]
/// This trait is designed to provide a way to set the log level, when the option
/// or the flag doesn't have a default level and the `simplelog` feature is eanbled
pub trait SetLogWithDefault: GetWithDefault {
    /// This function can be used to set the loglevel
    /// if the feature `simplelog` is enabled
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
