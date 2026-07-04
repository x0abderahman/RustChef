use super::{Operation, OperationResult};

pub struct Base64Encode;
pub struct Base64Decode;

impl Operation for Base64Encode {
    fn name(&self) -> &'static str {
        "base64-encode"
    }

    fn description(&self) -> &'static str {
        "Encode input data to Base64"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        use base64::Engine;
        let encoded = base64::engine::general_purpose::STANDARD.encode(input.as_bytes());
        Ok(OperationResult::new(encoded, "Base64 encoded".to_string()))
    }
}

impl Operation for Base64Decode {
    fn name(&self) -> &'static str {
        "base64-decode"
    }

    fn description(&self) -> &'static str {
        "Decode Base64 encoded data"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        use base64::Engine;
        let trimmed = input.trim();
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(trimmed)
            .map_err(|e| format!("Base64 decode error: {}", e))?;
        let output = String::from_utf8_lossy(&decoded).to_string();
        Ok(OperationResult::new(output, "Base64 decoded".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let op = Base64Encode;
        let result = op.perform("hello").unwrap();
        assert_eq!(result.output, "aGVsbG8=");
    }

    #[test]
    fn test_base64_decode() {
        let op = Base64Decode;
        let result = op.perform("aGVsbG8=").unwrap();
        assert_eq!(result.output, "hello");
    }

    #[test]
    fn test_base64_decode_with_whitespace() {
        let op = Base64Decode;
        let result = op.perform("  aGVsbG8=  ").unwrap();
        assert_eq!(result.output, "hello");
    }

    #[test]
    fn test_base64_decode_invalid() {
        let op = Base64Decode;
        let result = op.perform("!!!invalid!!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_base64_encode_empty() {
        let op = Base64Encode;
        let result = op.perform("").unwrap();
        assert_eq!(result.output, "");
    }

    #[test]
    fn test_base64_roundtrip() {
        let encode = Base64Encode;
        let decode = Base64Decode;
        let input = "Hello, World! 123 🎉";
        let encoded = encode.perform(input).unwrap();
        let decoded = decode.perform(&encoded.output).unwrap();
        assert_eq!(decoded.output, input);
    }
}
