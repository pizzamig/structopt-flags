# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- QuietVerbose: the example in the doc section was using Verbose
- QuietVerbose: fixed typo in the description of '-q'

## [0.3.1] - 2019-04-18
### Added
- ForceFlag: flag to support --force and -f

### Changed
- add features simplelog in docs.rs (hopefully)

## [0.3.0] - 2019-04-10
### Added
- features: add simplelog as an optional feature
- ConfigFile and ConfigFileNoDef: options to provide a configuration file

### Changed
- Make the GetWithDefault trait more idiomatic, using the Into trait

## [0.2.1] - 2018-11-02
### Added
- HostV4Param: ipv4 address mandatory parameter
- HostV6Param: ipv6 address mandatory parameter
- HostParam: ip address mandatory parameter

## [0.2.0] - 2018-10-15
### Added
- LogLevelOpt and LotLevelOptLower, to pass log level as -L or -l
- GetWithDefault trait, to allow users to define their own defaults
- VerboseNoDef, LogLevelNoDef, LogLevelNoDefLower: options with a user provided defaults
- HostV4Opt: first ipv4 address option
- HostV6Opt: first ipv6 address option
- HostOpt: first ip address option

### Fixed
- Fixing Cargo.toml metadata

## [0.1.0] - 2018-10-07
### Added
- Verbose and QuietVerbose implentation are added
- LogLevel trait, for all option providing a log setting
- Documentation
- SimpleVerbose implementation, a boolean verbose flag
