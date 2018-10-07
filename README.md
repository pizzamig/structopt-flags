# structopt-flags

[![Build Status](https://travis-ci.org/pizzamig/structopt-flags.svg)][Travis]
[Travis]: https://travis-ci.org/pizzamig/structopt-flags
[![Docs](https://docs.rs/structopt-flags/badge.svg)](https://docs.rs/structopt-flags)
[![dependency status](https://deps.rs/repo/github/pizzamig/structopt-flags/status.svg)](https://deps.rs/repo/github/pizzamig/structopt-flags)

A set of reusable flags and option for your CLIs using StructOpt

## Geting started

Add the crate to your project, adding this in `Cargo.toml`:

```toml
[dependencies]
structopt-flags = "0.1"
```

In your code, you can use one or more flags provided by this crate.
For instance:
```rust
extern crate failure;
extern crate structopt_flags;
#[macro_use]
extern crate structopt;

use failure::Error;
use structopt::StructOpt;
use structopt_flags::LogLevel; // traits for flags that can provide a log level

#[derive(Debug, StructOpt)]
#[structopt(name = "verbose", about = "An example using verbose flag")]
struct Opt {
    #[structopt(flatten)]
    verbose: structopt_flags::Verbose,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
	let _log_level = opt.verbose.get_level_filter();
	// set the log level of your preferred log crate
    Ok(())
}
```


## License

Licensed under:

 * BSD 3-Clause License ([LICENSE](LICENSE) or https://opensource.org/licenses/BSD-3-Clause )
