use crate::errors::{MakerError, MakerErrorType};

use super::{lexer::Location, values::RuntimeValue};
use std::collections::HashMap;

pub struct Scope {
    pub variables: HashMap<String, RuntimeValue>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Result<RuntimeValue, MakerError> {
        self.exists_current(name)?;
        Ok(self.variables.get(name).unwrap().clone())
    }

    fn exists_current(&self, name: &str) -> Result<(), MakerError> {
        if !self.variables.contains_key(name) {
            return Err(MakerError::lang(
                format!("The variable {} does not exist", name),
                Location::no_location(),
                MakerErrorType::RuntimeError,
            ));
        }

        Ok(())
    }

    pub fn declare(&mut self, name: &str, value: RuntimeValue) -> Result<RuntimeValue, MakerError> {
        if self.variables.contains_key(name) {
            return Err(MakerError::lang(
                format!("The variable {} already exists", name),
                Location::no_location(),
                MakerErrorType::RuntimeError,
            ));
        }

        // Declare it
        self.variables.insert(name.to_string(), value.clone());
        Ok(value)
    }
}
