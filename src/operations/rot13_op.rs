use super::{Operation, OperationResult};

pub struct ROT13;

impl Operation for ROT13 {
    fn name(&self) -> &'static str {
        "rot13"
    }

    fn description(&self) -> &'static str {
        "Apply ROT13 (Caesar cipher shift by 13) to the input"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let output = input
            .chars()
            .map(|c| match c {
                'a'..='z' => char::from((c as u8 - b'a' + 13) % 26 + b'a'),
                'A'..='Z' => char::from((c as u8 - b'A' + 13) % 26 + b'A'),
                _ => c,
            })
            .collect();
        Ok(OperationResult::new(output, "ROT13 applied".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13_basic() {
        let op = ROT13;
        let result = op.perform("hello").unwrap();
        assert_eq!(result.output, "uryyb");
    }

    #[test]
    fn test_rot13_roundtrip() {
        let op = ROT13;
        let input = "Hello, World!";
        let first = op.perform(input).unwrap();
        let second = op.perform(&first.output).unwrap();
        assert_eq!(second.output, input);
    }

    #[test]
    fn test_rot13_non_alpha() {
        let op = ROT13;
        let result = op.perform("123 !@#").unwrap();
        assert_eq!(result.output, "123 !@#");
    }

    #[test]
    fn test_rot13_uppercase() {
        let op = ROT13;
        let result = op.perform("ABC").unwrap();
        assert_eq!(result.output, "NOP");
    }
}
