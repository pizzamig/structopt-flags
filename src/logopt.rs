use super::{GetWithDefault, LogLevel};
use log::{Level, LevelFilter};
use std::fmt;

/// This struct provides the `--log` and `-L` cli option
///
/// By default, the log level is set to info.
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
/// #[structopt(name = "log_level_opt", about = "An example using the LogLevelOpt option")]
/// struct Opt {
///     #[structopt(flatten)]
///     log_level: structopt_flags::LogLevelOpt,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let filter_level = opt.log_level.get_level_filter();
///     // set log level
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct LogLevelOpt {
    /// Set the log level to run under
    /// Possible values are: off, error, warn, info, debug, trace
    #[structopt(
        name = "loglevel",
        long = "log-level",
        short = "L",
        default_value = "info",
        raw(global = "true"),
    )]
    log_level: LevelFilter,
}

impl LogLevel for LogLevelOpt {
    fn get_level_filter(&self) -> LevelFilter {
        self.log_level
    }

    fn get_log_level(&self) -> Option<Level> {
        self.get_level_filter().to_level()
    }
}

impl fmt::Display for LogLevelOpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.log_level)
    }
}

/// This struct provides the `--log` and `-l` cli option
///
/// By default, the log level is set to info.
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
/// #[structopt(name = "log_level_opt_lower", about = "An example using the LogLevelOptLower option")]
/// struct Opt {
///     #[structopt(flatten)]
///     log_level: structopt_flags::LogLevelOptLower,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let filter_level = opt.log_level.get_level_filter();
///     // set log level
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct LogLevelOptLower {
    /// Set the log level to run under
    /// Possible values are: off, error, warn, info, debug, trace
    #[structopt(
        name = "loglevellower",
        long = "log-level",
        short = "l",
        default_value = "info",
        raw(global = "true"),
        conflicts_with = "loglevel"
    )]
    log_level: LevelFilter,
}

impl LogLevel for LogLevelOptLower {
    fn get_level_filter(&self) -> LevelFilter {
        self.log_level
    }

    fn get_log_level(&self) -> Option<Level> {
        self.get_level_filter().to_level()
    }
}

impl fmt::Display for LogLevelOptLower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.log_level)
    }
}

/// This struct provides the `--log` and `-L` cli option, with no default
///
/// No default value is provided
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
/// #[structopt(name = "log_level_no_def", about = "An example using the LogLevelNoDef option")]
/// struct Opt {
///     #[structopt(flatten)]
///     log_level: structopt_flags::LogLevelNoDef,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let filter_level = opt.log_level.get_with_default(LevelFilter::Warn);
///     // set log level
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct LogLevelNoDef {
    /// Set the log level to run under
    /// Possible values are: off, error, warn, info, debug, trace
    #[structopt(
        name = "loglevel",
        long = "log-level",
        short = "L",
        raw(global = "true"),
    )]
    log_level: Option<LevelFilter>,
}

impl GetWithDefault for LogLevelNoDef {
    type Item = LevelFilter;
    fn get_with_default(&self, default: Self::Item) -> Self::Item {
        self.log_level.unwrap_or(default)
    }
}

impl fmt::Display for LogLevelNoDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.log_level {
            Some(x) => write!(f, "{}", x),
            None => write!(f, "None"),
        }
    }
}

/// This struct provides the `--log` and `-l` cli option, with no default
///
/// No default value is provided
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
/// #[structopt(name = "log_level_no_def_lower", about = "An example using the LogLevelNoDefLower option")]
/// struct Opt {
///     #[structopt(flatten)]
///     log_level: structopt_flags::LogLevelNoDefLower,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let filter_level = opt.log_level.get_with_default(LevelFilter::Warn);
///     // set log level
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct LogLevelNoDefLower {
    /// Set the log level to run under
    /// Possible values are: off, error, warn, info, debug, trace
    #[structopt(
        name = "loglevel",
        long = "log-level",
        short = "l",
        raw(global = "true"),
    )]
    log_level: Option<LevelFilter>,
}

impl GetWithDefault for LogLevelNoDefLower {
    type Item = LevelFilter;
    fn get_with_default(&self, default: Self::Item) -> Self::Item {
        self.log_level.unwrap_or(default)
    }
}

impl fmt::Display for LogLevelNoDefLower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.log_level {
            Some(x) => write!(f, "{}", x),
            None => write!(f, "None"),
        }
    }
}
