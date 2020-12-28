# structopt-flags

[![Rust](https://github.com/pizzamig/structopt-flags/workflows/Rust/badge.svg)](https://github.com/pizzamig/structopt-flags/actions?query=workflow%3ARust)

[![Docs](https://docs.rs/structopt-flags/badge.svg)](https://docs.rs/structopt-flags)
[![dependency status](https://deps.rs/repo/github/pizzamig/structopt-flags/status.svg)](https://deps.rs/repo/github/pizzamig/structopt-flags)

A set of reusable flags and option for your CLIs using StructOpt

## Getting started

Add the crate to your project, adding this in `Cargo.toml`:

```toml
[dependencies]
structopt-flags = "0.3"
```

In your code, you can use one or more flags provided by this crate.
For instance:
```rust
extern crate structopt;
extern crate structopt_flags;

use structopt::StructOpt;
use structopt_flags::LogLevel; // traits for flags that can provide a log level

#[derive(Debug, StructOpt)]
#[structopt(name = "verbose", about = "An example using verbose flag")]
struct Opt {
    #[structopt(flatten)]
    verbose: structopt_flags::Verbose,
}

fn main() {
    let opt = Opt::from_args();
	let _log_level = opt.verbose.get_level_filter();
	// set the log level of your preferred log crate
}
```

## `simplelog` Feature

A non-default feature added in version 0.3 is an API to automatically configure `simplelog` depending on you log or verbosity level.
To use this feature, add in your `Cargo.toml`

```toml
[dependencies]
structopt-flags = { version = "0.3", features = ["simplelog"] }
```

## License

Licensed under:

 * BSD 3-Clause License ([LICENSE](LICENSE) or https://opensource.org/licenses/BSD-3-Clause )
