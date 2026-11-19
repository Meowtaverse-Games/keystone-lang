use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    MismatchedTypes {
        op: Op,
        left: Type,
        right: Type,
    },
    InvalidOperandType {
        op: Op,
        typ: Type,
    },
    InvalidUnaryOperandType {
        op: UnaryOp,
        typ: Type,
    },
    UnexpectedType {
        statement: String,
        found_type: Type,
    },
    SyntaxError {
        messages: Vec<String>,
    },
    NameError {
        name: String,
    },
    /// A variable keeps the type of its first assignment.
    TypeChanged {
        name: String,
        from: Type,
        to: Type,
    },
    ArgError {
        called: String,
        expected: u8,
        got: u8,
    },
    CommandNotAllowed {
        command: String,
    },
    ZeroDivisionError,
    TooLargeNumber,
}
