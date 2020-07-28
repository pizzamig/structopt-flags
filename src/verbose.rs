use crate::{GetWithDefault, LogLevel};
use log::LevelFilter;
use std::fmt;
use structopt::StructOpt;

#[cfg(feature = "simplelog")]
use crate::SetLogWithDefault;
#[cfg(feature = "simplelog")]
use simplelog::{Config, TermLogger, TerminalMode};
#[derive(StructOpt, Debug, Clone)]

pub struct Verbose {
    /// Increase the output's verbosity level (default:info)
    /// Pass many times to increase verbosity level, up to 4.
    #[structopt(
        name = "verbose",
        long = "verbose",
        short = "v",
        parse(from_occurrences),
        global = true
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
}

impl fmt::Display for Verbose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_level_filter())
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct VerboseNoDef {
    /// Increase the output's verbosity level
    /// Pass many times to increase verbosity level.
    #[structopt(
        name = "verbose",
        long = "verbose",
        short = "v",
        parse(from_occurrences),
        global = true
    )]
    verbosity_level: u8,
}

impl GetWithDefault for VerboseNoDef {
    type Item = LevelFilter;
    fn get_with_default<T: Into<Self::Item>>(&self, default: T) -> Self::Item {
        let default_value: i8 = match default.into() {
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
        TermLogger::init(
            self.get_with_default(default),
            Config::default(),
            TerminalMode::Mixed,
        )
        .unwrap_or(());
    }
}
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
        global = true
    )]
    verbosity_level: u8,

    /// Decrease the output's verbosity level
    /// Used once, it will set error log level.
    /// Used twice, will slient the log completely
    #[structopt(
        name = "quietquiet",
        long = "quiet",
        short = "q",
        parse(from_occurrences),
        conflicts_with = "quietverbose",
        global = true
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
}

impl fmt::Display for QuietVerbose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_level_filter())
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct SimpleVerbose {
    /// Enable the verbose output
    /// No multiple occurrences are supported
    #[structopt(name = "simpleverbose", long = "verbose", short = "v", global = true)]
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
