use std::fmt::{Display, Formatter};

/// The EquationError struct is used to represent an error that can occur in the equation solver.
#[derive(Debug, Clone, PartialEq)]
pub struct EquationError {
    /// The message of the error.
    pub message: String,
    /// The type of the error.
    pub type_: EquationErrorType,
}

impl EquationError {
    /// Creates a new EquationError from the message and the type of error.
    pub fn new(message: String, type_: EquationErrorType) -> EquationError {
        EquationError { message, type_ }
    }
}

/// The EquationErrorType enum is used to represent the type of error that can occur in the equation solver.
#[derive(Debug, Clone, PartialEq)]
pub enum EquationErrorType {
    /// Items were missing in the equation.
    MissingItems,
    /// An unexpected token was found in the equation.
    UnexpectedToken,
    /// An unset variable was found.
    UnsetVariable,
}

impl Display for EquationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.type_, self.message)
    }
}

impl std::error::Error for EquationError {
    fn description(&self) -> &str {
        &self.message
    }
}
