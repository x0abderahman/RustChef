use super::{Operation, OperationResult};

pub struct XOR {
    pub key: String,
}

impl Operation for XOR {
    fn name(&self) -> &'static str {
        "xor"
    }

    fn description(&self) -> &'static str {
        "Apply XOR operation with a key to the input"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        if self.key.is_empty() {
            return Err("XOR key cannot be empty".to_string());
        }
        let key_bytes = self.key.as_bytes();
        let output = input
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(i, b)| (b ^ key_bytes[i % key_bytes.len()]) as char)
            .collect();
        Ok(OperationResult::new(output, "XOR applied".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_basic() {
        let op = XOR { key: "key".to_string() };
        let result = op.perform("hello").unwrap();
        assert_eq!(result.output.len(), 5);
    }

    #[test]
    fn test_xor_roundtrip() {
        let op1 = XOR { key: "secret".to_string() };
        let input = "Hello, World!";
        let encrypted = op1.perform(input).unwrap();
        let op2 = XOR { key: "secret".to_string() };
        let decrypted = op2.perform(&encrypted.output).unwrap();
        assert_eq!(decrypted.output, input);
    }

    #[test]
    fn test_xor_empty_key() {
        let op = XOR { key: "".to_string() };
        let result = op.perform("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_xor_single_char_key() {
        let op = XOR { key: "A".to_string() };
        let result = op.perform("hello").unwrap();
        assert_eq!(result.output.len(), 5);
        // Double XOR with same key should return original
        let op2 = XOR { key: "A".to_string() };
        let result2 = op2.perform(&result.output).unwrap();
        assert_eq!(result2.output, "hello");
    }
}
