use std::collections::HashMap;

use crate::ast::{Expr, Type};

#[derive(Debug)]
pub struct TypeContext {
    pub vars: HashMap<String, Type>
}

impl TypeContext {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn set(&mut self, name: &str, typ: Type) {
        self.vars.insert(name.to_string(), typ);
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.vars.get(name)
    }
}

#[derive(Debug,Clone)]
pub struct RuntimeContext {
    pub vars: HashMap<String, Expr>
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn set(&mut self, name: &str, expr: Expr) {
        self.vars.insert(name.to_string(), expr);
    }

    pub fn get(&self, name: &str) -> &Expr {
        match self.vars.get(name){
            Some(x) => x,
            None => unreachable!("undefined variable")
        }
    }
}
