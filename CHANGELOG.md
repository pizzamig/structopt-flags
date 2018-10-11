# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- LogLevelOpt and LotLevelOptLower, to pass log level as -L or -l
- GetWithDefault trait, to allow users to define their own defaults
- VerboseNoDef, LogLevelNoDef, LogLevelNoDefLower: options with a user provided defaults
- HostV4Opt: first ipv4 address option

### Fixed
- Fixing Cargo.toml metadata

## [0.1.0] - 2018-10-07
### Added
- Verbose and QuietVerbose implentation are added
- LogLevel trait, for all option providing a log setting
- Documentation
- SimpleVerbose implementation, a boolean verbose flag
