//! A `cargo script` to validate an OBO file.
//!
//! ```cargo
//! [dependencies]
//! colored = "1.7.0"
//! clap = "2.33.0"
//! fastobo = "0.1.0"
//! ```

extern crate clap;
extern crate colored;
extern crate fastobo;

use colored::*;
use fastobo::ast::OboDoc;

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .author(clap::crate_authors!(","))
        .version(clap::crate_version!())
        .about(clap::crate_description!())

        .arg(clap::Arg::with_name("INPUT").required(true).multiple(true))
        //      .short("c")
        //      .long("config")
        //      .value_name("FILE")
        //      .help("Sets a custom config file")
        //      .takes_value(true))
        // .arg(Arg::with_name("INPUT")
        //      .help("Sets the input file to use")
        //      .required(true)
        //      .index(1))
        // .arg(Arg::with_name("v")
        //      .short("v")
        //      .multiple(true)
        //      .help("Sets the level of verbosity"))
        // .subcommand(SubCommand::with_name("test")
        //             .about("controls testing features")
        //             .version("1.3")
        //             .author("Someone E. <someone_else@other.com>")
        //             .arg(Arg::with_name("debug")
        //                 .short("d")
        //                 .help("print debug information verbosely")))
        .get_matches();


    
    let mut failures = Vec::new();


    for input in matches.values_of("INPUT").unwrap() {
        let relpath = std::path::PathBuf::from(input);
        let abspath = match std::fs::canonicalize(&relpath) {
            Ok(p) => p,
            Err(e) => {
                println!(
                    "{:>12} resolving path {}",
                    "Failed".red().bold(),
                    relpath.display(),
                );
                failures.push((relpath, fastobo::error::Error::from(e)));
                continue;
            }
        };

        let _doc = match OboDoc::from_file(&abspath) {
            Ok(d) => {
                println!(
                    "{:>12} parsing {} ({})",
                    "Finished".green().bold(),
                    relpath.display(),
                    abspath.display()
                );
                d
            }
            Err(e) => {
                println!(
                    "{:>12} parsing {} ({})",
                    "Failed".red().bold(),
                    relpath.display(),
                    abspath.display()
                );
                failures.push((relpath, e));
                continue;
            }
        };
    }

    if !failures.is_empty() {
        for (path, error) in failures {
            println!("{}", error)
        }
        std::process::exit(1);
    }

    // // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    // println!("Value for config: {}", config);
    //
    // // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // // required we could have used an 'if let' to conditionally get the value)
    // println!("Using input file: {}", matches.value_of("INPUT").unwrap());
    //
    // // Vary the output based on how many times the user used the "verbose" flag
    // // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }
    //
    // // You can handle information about subcommands by requesting their matches by name
    // // (as below), requesting just the name used, or both at the same time
    // if let Some(matches) = matches.subcommand_matches("test") {
    //     if matches.is_present("debug") {
    //         println!("Printing debug info...");
    //     } else {
    //         println!("Printing normally...");
    //     }
    // }
    //
    // // more program logic goes here...
}
