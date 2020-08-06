# `fastobo-validator` [![Star me](https://img.shields.io/github/stars/fastobo/fastobo-validator.svg?style=social&label=Star&maxAge=3600)](https://github.com/fastobo/fastobo-validator/stargazers)

*Faultess validation tool for OBO products.*

[![TravisCI](https://img.shields.io/travis/com/fastobo/fastobo-validator/master.svg?maxAge=600&style=flat-square)](https://travis-ci.com/fastobo/fastobo-validator/branches)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/fastobo-validator)
[![Binaries](https://img.shields.io/badge/binaries-bintray-brightgreen.svg?style=flat-square&maxAge=2678400)](https://bintray.com/fastobo/fastobo-validator)
[![Crate](https://img.shields.io/crates/v/fastobo-validator.svg?maxAge=600&style=flat-square)](https://crates.io/crates/fastobo-validator)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/fastobo-validator/blob/master/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/fastobo/fastobo-validator.svg?style=flat-square)](https://github.com/fastobo/fastobo-validator/issues)


## Overview

`fastobo-validator` is a command-line tool to validate an OBO file in format
version 1.4 against the [latest specification](http://owlcollab.github.io/oboformat/doc/obo-syntax.html).

## Setup

`fastobo-validator` is distributed as a pre-built binary for the following platforms:
* Linux x86-64 - 
  [stable version](https://bintray.com/fastobo/fastobo-validator/download_file?file_path=stable%2Ffastobo_validator-x86_64-linux-musl.tar.gz) -
  [development version](https://bintray.com/fastobo/fastobo-validator/download_file?file_path=master%2Ffastobo_validator-x86_64-linux-musl.tar.gz)
* OSX x86-64 - 
  [stable version](https://bintray.com/fastobo/fastobo-validator/download_file?file_path=stable%2Ffastobo_validator-x86_64-apple-darwin.tar.gz) -
  [development version](https://bintray.com/fastobo/fastobo-validator/download_file?file_path=master%2Ffastobo_validator-x86_64-apple-darwin.tar.gz)

Simply download the archive, and unpack the `fastobo-validator` binary somewhere in your `$PATH`. 
For other OS (notably Windows), you'll need to build the binary from source. Make sure to have the
Rust compiler installed (check the [installation methods](https://forge.rust-lang.org/other-installation-methods.html)) 
and simply run `cargo install fastobo-validator` to install the binary in your `$CARGO_HOME` folder.

## Validation

### Mandatory

#### Syntax

The syntax of the OBO format version 1.4 has been made more restrictive compared
to the format version 1.2, but files produces by modern tools (such as `ROBOT`)
should already be compliant with this version.


### Optional

#### ISBN validation (`-I`)

ISBN identifiers embed a validation digit which can be used to validate a given
code without querying an external database. Enabling this validation check will
process all `ISBN`-prefixed identifiers for a valid ISBN. It will *not* check
`ISBN10` or `ISBN13`-prefixed identifiers.


## Usage

Simply run the binary against one or more OBO files:
```console
$ fastobo-validator go.obo
```

The validator will then parse and validate each OBO product, and return with a
non-null error code if any error was detected, displaying a small report for
each error.


## Feedback

Found a bug ? Have an enhancement request ? Head over to the
[GitHub issue tracker](https://github.com/fastobo/fastobo-validator/issues) of the project if
you need to report or ask something. If you are filling in on a bug, please include as much
information as you can about the issue, and try to recreate the same bug in a simple, easily
reproducible situation.


## About

This project is currently being developed by [Martin Larralde](https://github.com/althonos)
as part of a Master's Degree internship in the [BBOP team](http://berkeleybop.org/) of the
[Lawrence Berkeley National Laboratory](https://www.lbl.gov/), under the supervision of
[Chris Mungall](http://biosciences.lbl.gov/profiles/chris-mungall/).
