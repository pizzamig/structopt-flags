use super::GetWithDefault;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct HostV4Opt {
    /// Set the host IP (Ipv4 only)
    #[structopt(name = "hostv4", long = "host", short = "-H", global = true)]
    host_addr: Option<Ipv4Addr>,
}

impl GetWithDefault for HostV4Opt {
    type Item = Ipv4Addr;
    fn get_with_default<T: Into<Self::Item>>(&self, default: T) -> Self::Item {
        self.host_addr.unwrap_or_else(|| default.into())
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct HostV4Param {
    /// Set the host IP (Ipv4 only)
    #[structopt(name = "hostv4", long = "host", short = "-H")]
    pub host_addr: Ipv4Addr,
}

#[derive(StructOpt, Debug, Clone)]
pub struct HostV6Opt {
    /// Set the host IP (Ipv6 only)
    #[structopt(name = "hostv6", long = "host", short = "-H", global = true)]
    host_addr: Option<Ipv6Addr>,
}

impl GetWithDefault for HostV6Opt {
    type Item = Ipv6Addr;
    fn get_with_default<T: Into<Self::Item>>(&self, default: T) -> Self::Item {
        self.host_addr.unwrap_or_else(|| default.into())
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct HostV6Param {
    /// Set the host IP (Ipv6 only)
    #[structopt(name = "hostv6", long = "host", short = "-H")]
    pub host_addr: Ipv6Addr,
}

#[derive(StructOpt, Debug, Clone)]
pub struct HostOpt {
    /// Set the host IP (both IpV4 and IpV6 are supported)
    #[structopt(name = "host", long = "host", short = "-H", global = true)]
    host_addr: Option<IpAddr>,
}

impl GetWithDefault for HostOpt {
    type Item = IpAddr;
    fn get_with_default<T: Into<Self::Item>>(&self, default: T) -> Self::Item {
        self.host_addr.unwrap_or_else(|| default.into())
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct HostParam {
    /// Set the host IP (both IpV4 and IpV6 are supported)
    #[structopt(name = "host", long = "host", short = "-H")]
    pub host_addr: IpAddr,
}
