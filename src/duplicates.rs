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
pub struct DuplicateIdError {
    id: Ident,
    count: usize
}

impl Error for DuplicateIdError {
    fn description(&self) -> &str {
        "id appears more than once"
    }
}

impl Display for DuplicateIdError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "id `{}` appears more than once ({} times)", self.id, self.count)
    }
}

#[derive(Default)]
pub struct DuplicateIdChecker<'a> {
    counts: HashMap<&'a Ident, usize>,
}

macro_rules! impl_visit {
    ($name:ident, $frame:ty) => {
        fn $name(&mut self, frame: &'a $frame) {
            *self.counts.entry(frame.as_id()).or_default() += 1;
        }
    }
}

impl<'a> Visit<'a> for DuplicateIdChecker<'a> {
    impl_visit!(visit_term_frame, TermFrame);
    impl_visit!(visit_typedef_frame, TypedefFrame);
    impl_visit!(visit_instance_frame, InstanceFrame);
}

impl Validator for DuplicateIdChecker<'_> {
    fn validate(doc: &OboDoc) -> Vec<ValidationError> {
        let mut checker = Self::default();
        checker.visit_doc(&doc);

        let mut errors = Vec::new();
        for (id, count) in checker.counts {
            if count > 1 {
                errors.push(ValidationError {
                    location: String::from("complete document"),
                    cause: Box::new(DuplicateIdError {
                        id: id.clone(),
                        count
                    }),
                })
            }
        }

        errors
    }
}
