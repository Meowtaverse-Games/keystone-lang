use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Error {
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
    SyntaxError {
        messages: Vec<String>
    },
    NameError {
        name: String
    },
    ZeroDivisionError
}