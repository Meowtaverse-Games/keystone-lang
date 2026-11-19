use crate::ast::{Expr, Type};
use crate::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TypeContext {
    pub vars: HashMap<String, Type>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, typ: Type) {
        self.vars.insert(name.to_string(), typ);
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.vars.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeContext {
    pub vars: HashMap<String, Expr>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, expr: Expr) {
        self.vars.insert(name.to_string(), expr);
    }

    /// The type checker should make this unreachable; report it rather than panic if not.
    pub fn get(&self, name: &str) -> Result<&Expr, Error> {
        self.vars.get(name).ok_or_else(|| Error::NameError {
            name: name.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_an_unset_runtime_variable_is_an_error() {
        assert_eq!(
            RuntimeContext::new().get("y"),
            Err(Error::NameError {
                name: "y".to_string()
            })
        );
    }
}
