//! Faultess validation tool for OBO products.

extern crate clap;
extern crate colored;
extern crate fastobo;
extern crate isbn as isbn_crate;
extern crate itertools;
extern crate textwrap;

pub mod cardinality;
pub mod isbn;
pub mod duplicates;

use std::error::Error;
use fastobo::ast::*;

pub use self::cardinality::CardinalityChecker;
pub use self::duplicates::DuplicateIdChecker;
pub use self::isbn::IsbnChecker;

pub trait Validator {
    fn validate(doc: &OboDoc) -> Vec<ValidationError>;
}

pub struct ValidationError {
    pub location: String,
    pub cause: Box<dyn Error>,
}
