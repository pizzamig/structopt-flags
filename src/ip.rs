use super::GetWithDefault;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// This struct provides the `--host` and `-H` cli option to get an IPv4 address
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
/// #[structopt(name = "ipv4_opt", about = "An example using the HostV4Opt option")]
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
    /// Set the host IP (Ipv4 only)
    #[structopt(
        name = "hostv4",
        long = "host",
        short = "-H",
        raw(global = "true")
    )]
    host_addr: Option<Ipv4Addr>,
}

impl GetWithDefault for HostV4Opt {
    type Item = Ipv4Addr;
    fn get_with_default(&self, default: Self::Item) -> Self::Item {
        self.host_addr.unwrap_or(default)
    }
}

/// This struct provides the `--host` and `-H` cli option to get an IPv6 address
///
/// No default is provided
///
/// ```rust
/// extern crate structopt_flags;
/// #[macro_use]
/// extern crate structopt;
///
/// use std::net::Ipv6Addr;
/// use structopt::StructOpt;
/// use structopt_flags::GetWithDefault; // to access get_ipv4_addr
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "ipv6_opt", about = "An example using the HostV6Opt option")]
/// struct Opt {
///     #[structopt(flatten)]
///     host_ip: structopt_flags::HostV6Opt,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let ipv6 = opt.host_ip.get_with_default(Ipv6Addr::new(0,0,0,0,0,0,0,1));
/// }
/// ```

#[derive(StructOpt, Debug, Clone)]
pub struct HostV6Opt {
    /// Set the host IP (Ipv6 only)
    #[structopt(
        name = "hostv6",
        long = "host",
        short = "-H",
        raw(global = "true")
    )]
    host_addr: Option<Ipv6Addr>,
}

impl GetWithDefault for HostV6Opt {
    type Item = Ipv6Addr;
    fn get_with_default(&self, default: Self::Item) -> Self::Item {
        self.host_addr.unwrap_or(default)
    }
}

/// This struct provides the `--host` and `-H` cli option to get ageneric IP address
///
/// No default is provided
///
/// ```rust
/// extern crate structopt_flags;
/// #[macro_use]
/// extern crate structopt;
///
/// use std::net::{IpAddr,Ipv6Addr};
/// use structopt::StructOpt;
/// use structopt_flags::GetWithDefault; // to access get_ipv4_addr
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "ip_opt", about = "An example using the HostOpt option")]
/// struct Opt {
///     #[structopt(flatten)]
///     host_ip: structopt_flags::HostOpt,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     let ip = opt.host_ip.get_with_default(IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1)));
/// }
/// ```

#[derive(StructOpt, Debug, Clone)]
pub struct HostOpt {
    /// Set the host IP (both IpV4 and IpV6 are supported)
    #[structopt(
        name = "host",
        long = "host",
        short = "-H",
        raw(global = "true")
    )]
    host_addr: Option<IpAddr>,
}

impl GetWithDefault for HostOpt {
    type Item = IpAddr;
    fn get_with_default(&self, default: Self::Item) -> Self::Item {
        self.host_addr.unwrap_or(default)
    }
}