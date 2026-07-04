pub mod base64_op;
pub mod hex_op;
pub mod url_op;
pub mod rot13_op;
pub mod xor_op;
pub mod hash_op;
pub mod extract_op;
pub mod text_op;
pub mod binary_op;
pub mod json_op;
pub mod case_op;

use std::fmt;

/// Represents the result of an operation
#[derive(Debug, Clone)]
pub struct OperationResult {
    pub output: String,
    pub description: String,
}

impl OperationResult {
    pub fn new(output: String, description: String) -> Self {
        Self { output, description }
    }
}

/// Trait that all operations must implement
pub trait Operation {
    /// Human-readable name of the operation
    fn name(&self) -> &'static str;

    /// A short description of what the operation does
    fn description(&self) -> &'static str;

    /// Perform the operation on the input, returning a result or an error
    fn perform(&self, input: &str) -> Result<OperationResult, String>;
}

impl fmt::Display for dyn Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.name(), self.description())
    }
}
