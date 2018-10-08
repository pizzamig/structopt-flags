use super::GetOrDefault;
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
/// use structopt::StructOpt;
/// use structopt_flags::GetOrDefault; // to access get_ipv4_addr
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
///     let ipv4_default = "127.0.0.1".parse().unwrap();
///     let ipv4 = opt.host_ip.get_or_default(ipv4_default);
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

impl GetOrDefault<Ipv4Addr> for HostV4Opt {
    fn get_or_default(&self, default: Ipv4Addr) -> Ipv4Addr {
        match self.host_addr {
            Some(x) => x,
            None => default,
        }
    }
}
