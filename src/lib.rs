//! The `equation-solver` crate allows intaking a string and parsing it into an equation that can be evaluated.
//! Using this you can compile the equation into a quickly interpretable equation which can be used to evaluate the equation.
//! Equations follow PEMDAS rules.

#![warn(missing_docs)]
#![deny(missing_debug_implementations)]

/// The equation module contains the equation struct and all the items that can be used in an equation.
pub mod equation;
/// The error module contains all associated things to errors that can be yielded in any stage of the equation solver.
pub mod error;
/// The item module contains all the items that can appear in an equation.
pub mod item;
/// The parse module contains the parser which is used to take strings and turn them into equations.
pub mod parse;


pub use equation::Equation;
pub use error::{EquationError, EquationErrorType};
