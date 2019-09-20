use crate::GetWithDefault;
use std::fmt;
use std::path::PathBuf;
use structopt::StructOpt;

/// This struct provides the `--config_file` cli option
///
/// The option is mandatory and require a filename
///
/// ```should_panic
/// extern crate structopt_flags;
/// #[macro_use]
/// extern crate structopt;
///
/// use structopt::StructOpt;
/// use structopt_flags::ConfigFile; // to access get_log_level
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "config-file", about = "An example using config_file option")]
/// struct Opt {
///     #[structopt(flatten)]
///     config: structopt_flags::ConfigFile,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let config_file = opt.config.get_filename();
///     // use the config file
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct ConfigFile {
    /// Set the configuration file
    #[structopt(name = "config_file", long = "config", short = "c", parse(from_os_str))]
    filename: PathBuf,
}

impl ConfigFile {
    pub fn get_filename(&self) -> PathBuf {
        self.filename.clone()
    }
}

impl fmt::Display for ConfigFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.filename.to_str().unwrap_or("not unicode filename")
        )
    }
}

/// This struct provides the `--config_file` cli option
///
/// The option is no mandatory, but a default value can be provided wit the get_with_default()
///
/// ```rust
/// extern crate structopt_flags;
/// #[macro_use]
/// extern crate structopt;
///
/// use structopt::StructOpt;
/// use structopt_flags::GetWithDefault; // to access get_log_level
/// use std::path::PathBuf;
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "config-file", about = "An example using config_file option")]
/// struct Opt {
///     #[structopt(flatten)]
///     config: structopt_flags::ConfigFileNoDef,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let config_file = opt.config.get_with_default("config-file.toml");
///     // use the config file
/// }
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct ConfigFileNoDef {
    /// Set the configuration file
    #[structopt(
        name = "config_file",
        long = "config",
        short = "c",
        parse(from_os_str),
        global = true
    )]
    filename: Option<PathBuf>,
}

impl GetWithDefault for ConfigFileNoDef {
    type Item = PathBuf;
    fn get_with_default<T: Into<Self::Item>>(&self, default: T) -> Self::Item {
        match &self.filename {
            Some(x) => x.clone(),
            None => default.into(),
        }
    }
}

impl fmt::Display for ConfigFileNoDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.filename {
            Some(x) => write!(f, "{}", x.to_str().unwrap_or("not unicode filename")),
            None => write!(f, "None"),
        }
    }
}
