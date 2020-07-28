use crate::GetWithDefault;
use std::fmt;
use std::path::PathBuf;
use structopt::StructOpt;

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
