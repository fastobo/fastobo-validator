use std::collections::HashMap;

use fastobo::ast::*;
use fastobo::error::CardinalityError;
use fastobo::semantics::Identified;
use fastobo::semantics::OboFrame;
use fastobo::visit::Visit;

use super::ValidationError;
use super::Validator;

#[derive(Default)]
pub struct CardinalityChecker<'a> {
    header_error: Option<CardinalityError>,
    entity_errors: HashMap<&'a Ident, CardinalityError>,
}

macro_rules! impl_visit {
    ($name:ident, $frame:ty) => {
        fn $name(&mut self, frame: &'a $frame) {
            if let Err(e) = frame.cardinality_check() {
                self.entity_errors.insert(frame.as_id(), e);
            }
        }
    }
}

impl<'a> Visit<'a> for CardinalityChecker<'a> {
    impl_visit!(visit_term_frame, TermFrame);
    impl_visit!(visit_typedef_frame, TypedefFrame);
    impl_visit!(visit_instance_frame, InstanceFrame);

    fn visit_header_frame(&mut self, frame: &'a HeaderFrame) {
        self.header_error = frame.cardinality_check().err();
    }
}

impl Validator for CardinalityChecker<'_> {
    fn validate(doc: &OboDoc) -> Vec<ValidationError> {
        let mut checker = Self::default();
        checker.visit_doc(doc);

        let mut errors = Vec::new();

        if let Some(err) = checker.header_error {
            errors.push(ValidationError {
                location: String::from("header"),
                cause: Box::new(err),
            })
        }

        for (entity, err) in checker.entity_errors {
            errors.push(ValidationError {
                location: format!("frame {}", entity),
                cause: Box::new(err),
            })
        }

        errors
    }
}
