use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo::ast::*;
use fastobo::error::CardinalityError;
use fastobo::semantics::Identified;
use fastobo::semantics::OboFrame;
use fastobo::visit::Visit;

use super::ValidationError;
use super::Validator;

#[derive(Debug)]
pub struct ObsoletionError {
    id: Ident,
    replaced_by: usize,
    consider: usize,
}

impl Display for ObsoletionError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.replaced_by > 0 {
            write!(f, "non-obsolete entity contains {} `replaced_by` clause(s)", self.replaced_by)
        } else {
            write!(f, "non-obsolete entity contains {} `consider` clause(s)", self.consider)
        }
    }
}

impl Error for ObsoletionError {
    fn description(&self) -> &str  {
        "non-obsolete frame contains obsoletion clauses"
    }
}

#[derive(Default)]
pub struct ObsoletionChecker {
    errors: Vec<ObsoletionError>,
}

macro_rules! impl_visit {
    ($name:ident, $frame:ty, $clause:ident) => {
        fn $name(&mut self, frame: &'a $frame) {

            let mut obsolete = false;
            let mut replaced_by: Vec<&'a Ident> = Vec::new();
            let mut consider: Vec<&'a Ident> = Vec::new();

            for clause in frame.clauses() {
                match clause.as_inner() {
                    $clause::IsObsolete(true) => obsolete = true,
                    $clause::ReplacedBy(id) => {
                        replaced_by.push(id.as_ref().as_ref())
                    }
                    $clause::Consider(id) => {
                        consider.push(id.as_ref().as_ref())
                    }
                    _ => ()
                }
            }

            if (replaced_by.len() > 0 || consider.len() > 1) && !obsolete {
                self.errors.push(ObsoletionError {
                    id: frame.id().as_inner().as_ref().as_ref().clone(),
                    replaced_by: replaced_by.len(),
                    consider: consider.len(),
                })
            }
        }
    }
}

impl<'a> Visit<'a> for ObsoletionChecker {
    impl_visit!(visit_term_frame, TermFrame, TermClause);
    impl_visit!(visit_typedef_frame, TypedefFrame, TypedefClause);
    impl_visit!(visit_instance_frame, InstanceFrame, InstanceClause);
}

impl Validator for ObsoletionChecker {
    fn validate(doc: &OboDoc) -> Vec<ValidationError> {
        let mut checker = Self::default();
        checker.visit_doc(&doc);

        let mut errors = Vec::new();
        for err in checker.errors {
            errors.push(ValidationError {
                location: format!("frame {}", err.id),
                cause: Box::new(err),
            })
        }


        errors
    }
}
