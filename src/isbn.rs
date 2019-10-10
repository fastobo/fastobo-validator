use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str::FromStr;

use fastobo::ast::*;
use fastobo::semantics::Identified;
use fastobo::visit::Visit;
use isbn_crate::Isbn;

use super::ValidationError;
use super::Validator;

#[derive(Debug)]
pub struct InvalidIsbn(PrefixedIdent, isbn_crate::IsbnError);

impl Error for InvalidIsbn {
    fn description(&self) -> &str {
        "invalid isbn"
    }
}

impl Display for InvalidIsbn {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "invalid isbn `{}` ", self.0)?;
        f.write_str(match self.1 {
            isbn_crate::IsbnError::InvalidChecksum => "(invalid checksum)",
            isbn_crate::IsbnError::InvalidLength => "(invalid length)",
            isbn_crate::IsbnError::InvalidDigit => "(invalid digit)",
            isbn_crate::IsbnError::InvalidGroup => "(invalid group)",
            isbn_crate::IsbnError::InvalidConversion => "(invalid conversion)",
            isbn_crate::IsbnError::UndefinedRange => "(undefined range)",
        })
    }
}

#[derive(Default)]
pub struct IsbnChecker<'a> {
    current_entity: Option<&'a Ident>,
    valid: HashSet<&'a PrefixedIdent>,
    invalid: HashMap<&'a Ident, Vec<InvalidIsbn>>,
}

impl<'a> Visit<'a> for IsbnChecker<'a> {
    fn visit_entity_frame(&mut self, entity: &'a EntityFrame) {
        self.current_entity = Some(entity.as_id());
        match entity {
            EntityFrame::Term(t) => self.visit_term_frame(t),
            EntityFrame::Typedef(t) => self.visit_typedef_frame(t),
            EntityFrame::Instance(i) => self.visit_instance_frame(i),
        }
    }

    fn visit_prefixed_ident(&mut self, id: &'a PrefixedIdent) {
        if id.prefix().as_str() == "ISBN" {
            if let Err(e) = Isbn::from_str(id.local().as_str()) {
                self.invalid
                    .entry(self.current_entity.unwrap())
                    .or_default()
                    .push(InvalidIsbn(id.clone(), e));
            } else {
                self.valid.insert(id);
            }
        }
    }
}

impl Validator for IsbnChecker<'_> {
    fn validate(doc: &OboDoc) -> Vec<ValidationError> {
        let mut checker = Self::default();
        checker.visit_doc(&doc);

        let mut errors = Vec::new();
        for (entity, errs) in checker.invalid {
            for err in errs {
                errors.push(ValidationError {
                    location: format!("frame {}", entity),
                    cause: Box::new(err),
                })
            }
        }

        errors
    }
}
