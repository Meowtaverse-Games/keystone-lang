use crate::ast::*;

#[derive(Debug, Clone)]
pub enum TypeError {
    MismatchedTypes {
        op: Op,
        left: Type,
        right: Type
    },
    InvalidOperandType {
        op: Op,
        typ: Type
    },
    UnexpectedType {
        statement: String,
        found_type: Type,
    },
}