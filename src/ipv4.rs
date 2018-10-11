use super::GetWithDefault;
use super::Ipv4Address;
use std::fmt;
use std::net::Ipv4Addr;

/// This struct provides the `--host` and `-H` cli option
///
/// No default is provided
///
/// ```rust
/// extern crate structopt_flags;
/// #[macro_use]
/// extern crate structopt;
///
/// use std::net::Ipv4Addr;
/// use structopt::StructOpt;
/// use structopt_flags::GetWithDefault; // to access get_ipv4_addr
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "log_level_opt", about = "An example using the LogLevelOpt option")]
/// struct Opt {
///     #[structopt(flatten)]
///     host_ip: structopt_flags::HostV4Opt,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let ipv4 = opt.host_ip.get_with_default(Ipv4Addr::new(127,0,0,1));
/// }
/// ```

#[derive(StructOpt, Debug, Clone)]
pub struct HostV4Opt {
    /// Set the host IP (v4)
    #[structopt(
        name = "hostv4",
        long = "host",
        short = "-H",
        raw(global = "true")
    )]
    host_addr: Option<Ipv4Addr>,
}

impl Ipv4Address for HostV4Opt {
    fn get_ipv4_addr(&self) -> Option<Ipv4Addr> {
        self.host_addr
    }
}

impl fmt::Display for HostV4Opt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.host_addr.is_none() {
            write!(f, "None")
        } else {
            write!(f, "{}", self.host_addr.unwrap())
        }
    }
}

impl GetWithDefault for HostV4Opt {
    type Item = Ipv4Addr;
    fn get_with_default(&self, default: Self::Item) -> Self::Item {
        self.host_addr.unwrap_or(default)
    }
}
