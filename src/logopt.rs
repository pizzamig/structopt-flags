use crate::{GetWithDefault, LogLevel};
use log::LevelFilter;
use std::fmt;
use structopt::StructOpt;

#[cfg(feature = "simplelog")]
use crate::SetLogWithDefault;
#[cfg(feature = "simplelog")]
use simplelog::{Config, TermLogger, TerminalMode};
#[derive(StructOpt, Debug, Clone)]
pub struct LogLevelOpt {
    /// Set the log level to run under
    /// Possible values are: off, error, warn, info, debug, trace
    #[structopt(
        name = "loglevel",
        long = "log-level",
        short = "L",
        default_value = "info",
        global = true
    )]
    log_level: LevelFilter,
}

impl LogLevel for LogLevelOpt {
    fn get_level_filter(&self) -> LevelFilter {
        self.log_level
    }
}

impl fmt::Display for LogLevelOpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.log_level)
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct LogLevelOptLower {
    /// Set the log level to run under
    /// Possible values are: off, error, warn, info, debug, trace
    #[structopt(
        name = "loglevellower",
        long = "log-level",
        short = "l",
        default_value = "info",
        global = true,
        conflicts_with = "loglevel"
    )]
    log_level: LevelFilter,
}

impl LogLevel for LogLevelOptLower {
    fn get_level_filter(&self) -> LevelFilter {
        self.log_level
    }
}

impl fmt::Display for LogLevelOptLower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.log_level)
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct LogLevelNoDef {
    /// Set the log level to run under
    /// Possible values are: off, error, warn, info, debug, trace
    #[structopt(name = "loglevel", long = "log-level", short = "L", global = true)]
    log_level: Option<LevelFilter>,
}

impl GetWithDefault for LogLevelNoDef {
    type Item = LevelFilter;
    fn get_with_default<T: Into<Self::Item>>(&self, default: T) -> Self::Item {
        self.log_level.unwrap_or_else(|| default.into())
    }
}

#[cfg(feature = "simplelog")]
impl SetLogWithDefault for LogLevelNoDef {
    fn set_with_default(&self, default: LevelFilter) {
        TermLogger::init(
            self.get_with_default(default),
            Config::default(),
            TerminalMode::Mixed,
        )
        .unwrap_or(());
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

#[derive(StructOpt, Debug, Clone)]
pub struct LogLevelNoDefLower {
    /// Set the log level to run under
    /// Possible values are: off, error, warn, info, debug, trace
    #[structopt(name = "loglevel", long = "log-level", short = "l", global = true)]
    log_level: Option<LevelFilter>,
}

impl GetWithDefault for LogLevelNoDefLower {
    type Item = LevelFilter;
    fn get_with_default<T: Into<Self::Item>>(&self, default: T) -> Self::Item {
        self.log_level.unwrap_or_else(|| default.into())
    }
}

#[cfg(feature = "simplelog")]
impl SetLogWithDefault for LogLevelNoDefLower {
    fn set_with_default(&self, default: LevelFilter) {
        TermLogger::init(
            self.get_with_default(default),
            Config::default(),
            TerminalMode::Mixed,
        )
        .unwrap_or(());
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
