//! A `cargo script` to validate an OBO file.
//!
//! ```cargo
//! [dependencies]
//! clap = { version = "2.33.0", features = ["color"] }
//! colored = "1.7.0"
//! fastobo = "0.1.0"
//! failure = "0.1.5"
//! ```

extern crate clap;
extern crate colored;
extern crate fastobo;
#[macro_use]
extern crate failure;
extern crate isbn;
extern crate textwrap;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use colored::*;
use fastobo::ast::*;
use fastobo::error::Error as OboError;
use fastobo::visit::Visit;
use isbn::Isbn;
use isbn::IsbnError;

#[derive(Debug, Fail)]
enum ValidationError {
    #[fail(display = "{}", 0)]
    ParserFailed(OboError),
    #[fail(display = "invalid ISBN: `{}` ({:?})", 0, 1)]
    InvalidIsbn(PrefixedIdent, IsbnError),
}

#[derive(Default)]
struct IsbnChecker<'a> {
    valid: HashSet<&'a PrefixedIdent>,
    invalid: HashMap<&'a PrefixedIdent, IsbnError>,
}

impl<'a> Visit<'a> for IsbnChecker<'a> {
    fn visit_prefixed_ident(&mut self, id: &'a PrefixedIdent) {
        if id.prefix().as_str() == "ISBN" {
            if let Err(e) = Isbn::from_str(id.local().as_str()) {
                self.invalid.insert(id, e);
            } else {
                self.valid.insert(id);
            }
        }
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
                .multiple(true)
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
    let mut failures: HashMap<_, Vec<ValidationError>> = HashMap::new();

    for input in matches.values_of("INPUT").unwrap() {
        // Resolve the path.
        let path = std::path::PathBuf::from(input);
        failures.insert(path.clone(), Vec::new());

        // Parse the file
        println!("{:>12} `{}`", "Parsing".green().bold(), path.display());
        let start = std::time::Instant::now();
        let doc = match OboDoc::from_file(&path) {
            Ok(d) => {
                println!(
                    "{:>12} parsing `{}` in {:.2}s",
                    "Finished".green().bold(),
                    path.display(),
                    start.elapsed().as_millis() as f64 / 1000.0,
                );
                d
            }
            Err(e) => {
                println!("{:>12} parsing `{}`", "Failed".red().bold(), path.display());
                failures.entry(path).or_default().push(ValidationError::ParserFailed(e));
                continue;
            }
        };

        // Validate ISBN identifiers
        if matches.is_present("ISBN") {
            println!(
                "{:>12} ISBN codes in `{}`...",
                "Checking".green().bold(),
                &path.display()
            );

            let mut checker = IsbnChecker::default();
            checker.visit_doc(&doc);

            println!(
                "{:>12} {} ISBN codes",
                "Found".green().bold(),
                checker.valid.len() + checker.invalid.len()
            );

            for (id, err) in checker.invalid {
                failures.entry(path.clone()).or_default().push(ValidationError::InvalidIsbn(id.clone(), err));
            }
        }
    }

    // Display all errors and exit
    let mut retcode = 0;
    if !failures.is_empty() {
        for (path, errors) in failures {
            if errors.is_empty() {
                println!("{:>12} validation of `{}`", "Completed".green().bold(), path.display());
            } else {
                retcode = 1;
                println!("{:>12} validation of `{}` ({} errors)", "Failed".red().bold(), path.display(), errors.len());
                for error in errors {
                    print!("{}", textwrap::indent(&format!("{}", error), "             "));
                }
            }
        }
    }

    std::process::exit(retcode);
}
