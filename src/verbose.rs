use log::Level;
use std::fmt;

trait LogLevel {
    fn get_log_level(&self) -> log::Level;
}

#[derive(StructOpt, Debug, Clone)]
pub struct Verbose {
    /// Increase the output's verbosity level
    ///
    /// Pass many times to increase verbosity level
    #[structopt(
        long = "verbose",
        short = "v",
        parse(from_occurrences),
        raw(global = "true")
    )]
    verbosity_level: u8,
}

impl fmt::Display for Verbose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.verbosity_level)
    }
}

impl LogLevel for Verbose {
    fn get_log_level(&self) -> log::Level {
        match self.verbosity_level {
            0 => Level::Error,
            1 => Level::Warn,
            2 => Level::Info,
            3 => Level::Debug,
            _ => Level::Trace,
        }
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct QuietVerbose {
    /// Increase the output's verbosity level
    #[structopt(
        long = "verbose",
        short = "v",
        parse(from_occurrences),
        raw(global = "true")
    )]
    verbosity_level: u8,

    /// Decrease the output's verbosity level
    #[structopt(
        long = "quiet",
        short = "q",
        parse(from_occurrences),
        raw(global = "true")
    )]
    quiet_level: u8,
}

impl QuietVerbose {
    fn get_level(&self) -> u8 {
        let quiet = if self.quiet_level > 1 {
            2
        } else {
            self.quiet_level
        };
        let verbose = if self.verbosity_level > 3 {
            2
        } else {
            self.verbosity_level
        };
        verbose - quiet
    }
}

impl fmt::Display for QuietVerbose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.verbosity_level - self.quiet_level)
    }
}

impl LogLevel for QuietVerbose {
    fn get_log_level(&self) -> log::Level {
        match self.get_level() {
            0 => Level::Error,
            1 => Level::Warn,
            2 => Level::Info,
            3 => Level::Debug,
            _ => Level::Trace,
        }
    }
}
