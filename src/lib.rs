extern crate log;
#[macro_use]
extern crate structopt;

mod verbose;

pub use verbose::QuietVerbose;
pub use verbose::Verbose;
