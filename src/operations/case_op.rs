use super::{Operation, OperationResult};

pub struct LowerCase;
pub struct UpperCase;
pub struct Reverse;

impl Operation for LowerCase {
    fn name(&self) -> &'static str {
        "lower-case"
    }

    fn description(&self) -> &'static str {
        "Convert all characters to lowercase"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        Ok(OperationResult::new(
            input.to_lowercase(),
            "Converted to lowercase".to_string(),
        ))
    }
}

impl Operation for UpperCase {
    fn name(&self) -> &'static str {
        "upper-case"
    }

    fn description(&self) -> &'static str {
        "Convert all characters to uppercase"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        Ok(OperationResult::new(
            input.to_uppercase(),
            "Converted to uppercase".to_string(),
        ))
    }
}

impl Operation for Reverse {
    fn name(&self) -> &'static str {
        "reverse"
    }

    fn description(&self) -> &'static str {
        "Reverse the input string"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let output: String = input.chars().rev().collect();
        Ok(OperationResult::new(output, "Reversed".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase() {
        let op = LowerCase;
        let result = op.perform("Hello World!").unwrap();
        assert_eq!(result.output, "hello world!");
    }

    #[test]
    fn test_uppercase() {
        let op = UpperCase;
        let result = op.perform("Hello World!").unwrap();
        assert_eq!(result.output, "HELLO WORLD!");
    }

    #[test]
    fn test_reverse() {
        let op = Reverse;
        let result = op.perform("hello").unwrap();
        assert_eq!(result.output, "olleh");
    }

    #[test]
    fn test_reverse_palindrome() {
        let op = Reverse;
        let result = op.perform("racecar").unwrap();
        assert_eq!(result.output, "racecar");
    }

    #[test]
    fn test_reverse_empty() {
        let op = Reverse;
        let result = op.perform("").unwrap();
        assert_eq!(result.output, "");
    }

    #[test]
    fn test_lowercase_already_lower() {
        let op = LowerCase;
        let result = op.perform("hello").unwrap();
        assert_eq!(result.output, "hello");
    }
}
