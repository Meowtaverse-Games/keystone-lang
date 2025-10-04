use crate::ast::*;

#[derive(Debug, Clone)]
pub enum TypeError {
    MismatchedTypes {
        expected: Type,
        found: Type,
    },
    InvalidOperator {
        op: Op,
        lhs: Type,
        rhs: Type,
    }
}