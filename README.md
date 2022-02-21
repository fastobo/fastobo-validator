# `fastobo-validator` [![Star me](https://img.shields.io/github/stars/fastobo/fastobo-validator.svg?style=social&label=Star&maxAge=3600)](https://github.com/fastobo/fastobo-validator/stargazers)

*Faultess validation tool for OBO products.*

[![Actions](https://img.shields.io/github/workflow/status/fastobo/fastobo-validator/Test?style=flat-square&maxAge=600)](https://github.com/fastobo/fastobo-validator/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/fastobo-validator)
[![Binaries](https://img.shields.io/badge/binaries-linux--x86--64--musl-blue.svg?style=flat-square&maxAge=2678400)](https://github.com/fastobo/fastobo-validator/releases/latest/)
[![Crate](https://img.shields.io/crates/v/fastobo-validator.svg?maxAge=600&style=flat-square)](https://crates.io/crates/fastobo-validator)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/fastobo-validator/blob/master/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/fastobo/fastobo-validator.svg?style=flat-square)](https://github.com/fastobo/fastobo-validator/issues)


## Overview

`fastobo-validator` is a command-line tool to validate an OBO file in format
version 1.4 against the [latest specification](http://owlcollab.github.io/oboformat/doc/obo-syntax.html).

## Setup

`fastobo-validator` is distributed as a pre-built binary for the following platforms:
* Linux x86-64 -
  [latest version](https://github.com/fastobo/fastobo-validator/releases/latest/download/fastobo_validator-x86_64-linux-musl.tar.gz)
<!-- * OSX x86-64 -
  [latest version](https://bintray.com/fastobo/fastobo-validator/download_file?file_path=v0.4.0%2Ffastobo_validator-x86_64-apple-darwin.tar.gz) -->

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

#### Cardinality

Certain clauses (such as `name` or `def`) can only occur a fixed number of times
within a frame. `fastobo-validator` will check for the number of occurences of
those in the input document.


### Optional

#### ISBN validation (`-I` / `--ISBN`)

ISBN identifiers embed a validation digit which can be used to validate a given
code without querying an external database. Enabling this validation check will
process all `ISBN`-prefixed identifiers for a valid ISBN. It will *not* check
`ISBN10` or `ISBN13`-prefixed identifiers.


#### Frame duplication (`-d` / `--duplicates`)

While not forbidden by the OBO syntax and semantics, having frames with the same
ID in an OBO document is often an error. Use this flag to verify all frames
in the input have a unique identifier.


#### Obsoletion clauses (`-O` / `--obsoletion`)

Some clauses, such as `consider` or `replaced_by`, can only occur in frames for
entities that have been made obsolete. Use this flag to check this is the case.


#### All check (`--all`)

Enable all optional validation. *Note that using this parameter in an automated
context, such as a CI workflow, means that your file may not pass validation if
you update `fastobo-validator` after extra checks have been added. It is
recommended you only use this flag when running the binary yourself.*


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

This project was developed by [Martin Larralde](https://github.com/althonos)
as part of a Master's Degree internship in the [BBOP team](http://berkeleybop.org/) of the
[Lawrence Berkeley National Laboratory](https://www.lbl.gov/), under the supervision of
[Chris Mungall](http://biosciences.lbl.gov/profiles/chris-mungall/). Cite this project as:

*Larralde M.* **Developing Python and Rust libraries to improve the ontology ecosystem**
*\[version 1; not peer reviewed\].* F1000Research 2019, 8(ISCB Comm J):1500 (poster)
([https://doi.org/10.7490/f1000research.1117405.1](https://doi.org/10.7490/f1000research.1117405.1))
