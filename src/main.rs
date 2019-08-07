//! Faultess validation tool for OBO products.

extern crate clap;
extern crate colored;
#[macro_use]
extern crate err_derive;
extern crate fastobo;
extern crate isbn as isbn_crate;
extern crate itertools;
extern crate textwrap;

mod cardinality;
mod isbn;

extern crate fastobo_validator;

use colored::*;
use fastobo::ast::*;
use fastobo::error::Error;
use fastobo::error::SyntaxError;
use itertools::Itertools;

use fastobo_validator::Validator;
use fastobo_validator::ValidationError;
use fastobo_validator::cardinality::CardinalityChecker;
use fastobo_validator::isbn::IsbnChecker;

macro_rules! success {
    ($status:literal, $msg:literal, $($args:expr),*) => {
        println!(
            concat!("{:>12} ", $msg),
            $status.green().bold(),
            $($args),*
        )
    }
}

macro_rules! failure {
    ($status:literal, $msg:literal, $($args:expr),*) => {
        println!(
            concat!("{:>12} ", $msg),
            $status.red().bold(),
            $($args),*
        )
    }
}

fn main() {
    // Parse the CLI arguments.
    let matches = clap::App::new(clap::crate_name!())
        .author(clap::crate_authors!(","))
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("INPUT")
                .required(true)
                .help("The path to an OBO file"),
        )
        .arg(
            clap::Arg::with_name("ISBN")
                .short("I")
                .long("ISBN")
                .help("Enable syntactic validation of ISBN identifiers"),
        )
        .get_matches();

    // Record all failures
    let mut failures: Vec<ValidationError> = Vec::new();

    // Resolve the path.
    let path = std::path::PathBuf::from(matches.value_of("INPUT").unwrap());

    // Parse the file
    success!("Parsing", "`{}`", path.display());
    let start = std::time::Instant::now();
    let doc = match fastobo::from_file(&path) {
        Ok(d) => {
            let dt = start.elapsed().as_millis() as f64 / 1000.0;
            success!("Finished", "parsing `{}` in {:.2}s", path.display(), dt);
            d
        }
        Err(e) => {
            failure!("Failed", "parsing `{}`", path.display());
            if let Error::SyntaxError {
                error: SyntaxError::ParserError { error, .. },
                ..
            } = e
            {
                print!("{}", textwrap::indent(&error.to_string(), "        "))
            } else {
                print!("{}", textwrap::indent(&e.to_string(), "         --> "));
            }
            std::process::exit(1);
        }
    };

    // Mandatory validations
    failures.append(&mut CardinalityChecker::validate(&doc));

    // Optional validations
    if matches.is_present("ISBN") {
        failures.append(&mut IsbnChecker::validate(&doc));
    }

    // Display all errors and exit
    if !failures.is_empty() {
        failure!(
            "Failed",
            "validation of `{}` ({} errors)",
            path.display(),
            failures.len()
        );
        for (location, errors) in failures
            .into_iter()
            .group_by(|e| e.location.clone())
            .into_iter()
        {
            print!(
                "{}",
                textwrap::indent(&format!("in {}", location), "         --> ").bold()
            );
            for error in errors {
                print!(
                    "{}",
                    textwrap::indent(&format!("{}", error.cause), "             ")
                );
            }
        }
        std::process::exit(1);
    } else {
        success!("Completed", "validation of `{}`", path.display());
    }
}
