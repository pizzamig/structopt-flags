use super::LogLevel;
use log::Level;
use std::fmt;

/// This struct provides the `--verbose` cli option
///
/// By default, the log level is set to error.
/// Multiple occurrences, will increase the verbosity level up to 4.
///
/// ```rust
/// extern crate structopt_flags;
/// #[macro_use]
/// extern crate structopt;
///
/// use structopt::StructOpt;
/// use structopt_flags::LogLevel; // to access get_log_level
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "verbose", about = "An example using verbose flag")]
/// struct Opt {
///     #[structopt(flatten)]
///     verbose: structopt_flags::Verbose,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let log_level = opt.verbose.get_log_level().unwrap();
///     // set log level
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct Verbose {
    /// Increase the output's verbosity level (default:info)
    /// Pass many times to increase verbosity level, up to 4.
    #[structopt(
        long = "verbose",
        short = "v",
        parse(from_occurrences),
        raw(global = "true")
    )]
    verbosity_level: u8,
}

impl Verbose {
    fn get_level(&self) -> u8 {
        if self.verbosity_level > 3 {
            4
        } else {
            self.verbosity_level
        }
    }
}

impl fmt::Display for Verbose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_level())
    }
}

impl LogLevel for Verbose {
    fn get_log_level(&self) -> Option<Level> {
        match self.verbosity_level {
            0 => Some(Level::Error),
            1 => Some(Level::Warn),
            2 => Some(Level::Info),
            3 => Some(Level::Debug),
            _ => Some(Level::Trace),
        }
    }
}

/// This struct implements the `--verbose` and the `--quiet` cli options
///
/// By default, the log level is set to warning.
/// Multiple occurrences of `-v`, will increase the verbosity level up to 3.
/// The flag `-q` is used to decrease verbosity.
/// Using it twice, will silent the log.
///
/// ```rust
/// extern crate structopt_flags;
/// #[macro_use]
/// extern crate structopt;
///
/// use structopt::StructOpt;
/// use structopt_flags::LogLevel; // to access get_log_level
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "verbose", about = "An example using verbose flag")]
/// struct Opt {
///     #[structopt(flatten)]
///     verbose: structopt_flags::Verbose,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     if let Some(log_level) = opt.verbose.get_log_level() {
///         // set log level
///     }
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct QuietVerbose {
    /// Increase the output's verbosity level
    /// Pass many times to increase verbosity level, up to 3.
    #[structopt(
        name = "verbose",
        long = "verbose",
        short = "v",
        parse(from_occurrences),
        conflicts_with = "quiet",
        raw(global = "true")
    )]
    verbosity_level: u8,

    /// Decrease the output's verbosity level
    /// Used once, it will set error log level.
    /// Used twice, will slient te log completely
    #[structopt(
        name = "quiet",
        long = "quiet",
        short = "q",
        parse(from_occurrences),
        conflicts_with = "verbose",
        raw(global = "true")
    )]
    quiet_level: u8,
}

impl QuietVerbose {
    fn get_level(&self) -> i8 {
        let quiet = if self.quiet_level > 1 {
            2
        } else {
            self.quiet_level
        };
        let verbose = if self.verbosity_level > 2 {
            3
        } else {
            self.verbosity_level
        };
        verbose as i8 - quiet as i8
    }
}

impl fmt::Display for QuietVerbose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_level())
    }
}

impl LogLevel for QuietVerbose {
    fn get_log_level(&self) -> Option<Level> {
        match self.get_level() {
            -2 => None,
            -1 => Some(Level::Error),
            0 => Some(Level::Warn),
            1 => Some(Level::Info),
            2 => Some(Level::Debug),
            _ => Some(Level::Trace),
        }
    }
}
