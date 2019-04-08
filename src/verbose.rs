use crate::{GetWithDefault, LogLevel};
use log::{Level, LevelFilter};
use std::fmt;
use structopt::StructOpt;

#[cfg(feature = "simplelog")]
use crate::SetLogWithDefault;
#[cfg(feature = "simplelog")]
use simplelog::{Config, TermLogger};
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
///     let filter_level = opt.verbose.get_level_filter();
///     // set log level
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct Verbose {
    /// Increase the output's verbosity level (default:info)
    /// Pass many times to increase verbosity level, up to 4.
    #[structopt(
        name = "verbose",
        long = "verbose",
        short = "v",
        parse(from_occurrences),
        raw(global = "true")
    )]
    verbosity_level: u8,
}

impl LogLevel for Verbose {
    fn get_level_filter(&self) -> LevelFilter {
        match self.verbosity_level {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    }

    fn get_log_level(&self) -> Option<Level> {
        self.get_level_filter().to_level()
    }

    #[cfg(feature = "simplelog")]
    fn set_log_level(&self) {
        TermLogger::init(self.get_level_filter(), Config::default()).unwrap();
    }
}

impl fmt::Display for Verbose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_level_filter())
    }
}

/// This struct provides the `--verbose` cli option, without a predefined default
///
/// Multiple occurrences, will increase the verbosity level
///
/// ```rust
/// extern crate log;
/// extern crate structopt_flags;
/// #[macro_use]
/// extern crate structopt;
///
/// use log::LevelFilter;
/// use structopt::StructOpt;
/// use structopt_flags::GetWithDefault; // to access get_log_level
///
/// #[derive(Debug, StructOpt)]
/// #[structopt( name = "verbose_no_def", about = "An example using verbose flag with no predefined default")]
/// struct Opt {
///     #[structopt(flatten)]
///     verbose: structopt_flags::VerboseNoDef,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let filter_level = opt.verbose.get_with_default(LevelFilter::Off);
///     // set log level
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct VerboseNoDef {
    /// Increase the output's verbosity level
    /// Pass many times to increase verbosity level.
    #[structopt(
        name = "verbose",
        long = "verbose",
        short = "v",
        parse(from_occurrences),
        raw(global = "true")
    )]
    verbosity_level: u8,
}

impl GetWithDefault for VerboseNoDef {
    type Item = LevelFilter;
    fn get_with_default(&self, default: Self::Item) -> Self::Item {
        let default_value: i8 = match default {
            LevelFilter::Off => -1,
            LevelFilter::Error => 0,
            LevelFilter::Warn => 1,
            LevelFilter::Info => 2,
            LevelFilter::Debug => 3,
            LevelFilter::Trace => 4,
        };

        match default_value + self.verbosity_level as i8 {
            -1 => LevelFilter::Off,
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    }
}

#[cfg(feature = "simplelog")]
impl SetLogWithDefault for VerboseNoDef {
    fn set_with_default(&self, default: LevelFilter) {
        TermLogger::init(self.get_with_default(default), Config::default()).unwrap();
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
///     let filter_level = opt.verbose.get_level_filter();
///     // set log level
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct QuietVerbose {
    /// Increase the output's verbosity level
    /// Pass many times to increase verbosity level, up to 3.
    #[structopt(
        name = "quietverbose",
        long = "verbose",
        short = "v",
        parse(from_occurrences),
        conflicts_with = "quietquiet",
        raw(global = "true")
    )]
    verbosity_level: u8,

    /// Decrease the output's verbosity level
    /// Used once, it will set error log level.
    /// Used twice, will slient te log completely
    #[structopt(
        name = "quietquiet",
        long = "quiet",
        short = "q",
        parse(from_occurrences),
        conflicts_with = "quietverbose",
        raw(global = "true")
    )]
    quiet_level: u8,
}

impl LogLevel for QuietVerbose {
    fn get_level_filter(&self) -> LevelFilter {
        let quiet: i8 = if self.quiet_level > 1 {
            2
        } else {
            self.quiet_level as i8
        };
        let verbose: i8 = if self.verbosity_level > 2 {
            3
        } else {
            self.verbosity_level as i8
        };
        match verbose - quiet {
            -2 => LevelFilter::Off,
            -1 => LevelFilter::Error,
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    }
    fn get_log_level(&self) -> Option<Level> {
        self.get_level_filter().to_level()
    }
    #[cfg(feature = "simplelog")]
    fn set_log_level(&self) {
        TermLogger::init(self.get_level_filter(), Config::default()).unwrap();
    }
}

impl fmt::Display for QuietVerbose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_level_filter())
    }
}

/// This struct implements the `--verbose` cli option as a boolean flag
///
/// By default, the log level is set to warning.
/// Multiple occurrences of `-v` are not supported
///
/// ```rust
/// extern crate structopt_flags;
/// #[macro_use]
/// extern crate structopt;
///
/// use structopt::StructOpt;
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "verbose", about = "An example using verbose flag")]
/// struct Opt {
///     #[structopt(flatten)]
///     verbose: structopt_flags::SimpleVerbose,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     if opt.verbose.verbose {
///         println!("Verbose output enabled");
///     } else {
///         println!("No verbose output");
///     }
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct SimpleVerbose {
    /// Enable the verbose output
    /// No multiple occurrences are supported
    #[structopt(
        name = "simpleverbose",
        long = "verbose",
        short = "v",
        raw(global = "true")
    )]
    pub verbose: bool,
}

impl fmt::Display for SimpleVerbose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.verbose {
            write!(f, "True")
        } else {
            write!(f, "False")
        }
    }
}
