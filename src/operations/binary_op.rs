use super::{Operation, OperationResult};

pub struct BinaryEncode;
pub struct BinaryDecode;

impl Operation for BinaryEncode {
    fn name(&self) -> &'static str {
        "binary-encode"
    }

    fn description(&self) -> &'static str {
        "Encode input data to binary (bits)"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let output: String = input
            .as_bytes()
            .iter()
            .map(|b| format!("{:08b}", b))
            .collect::<Vec<_>>()
            .join(" ");
        Ok(OperationResult::new(output, "Binary encoded".to_string()))
    }
}

impl Operation for BinaryDecode {
    fn name(&self) -> &'static str {
        "binary-decode"
    }

    fn description(&self) -> &'static str {
        "Decode binary (bits) back to text"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let trimmed = input.trim();
        // Remove any whitespace and split into 8-bit chunks
        let binary_str: String = trimmed.chars().filter(|c| !c.is_whitespace()).collect();

        if !binary_str.len().is_multiple_of(8) {
            return Err("Binary string length must be a multiple of 8".to_string());
        }

        let bytes: Result<Vec<u8>, _> = (0..binary_str.len())
            .step_by(8)
            .map(|i| {
                let byte_str = &binary_str[i..i + 8];
                u8::from_str_radix(byte_str, 2)
                    .map_err(|e| format!("Invalid binary digit: {}", e))
            })
            .collect();

        let bytes = bytes?;
        let output = String::from_utf8_lossy(&bytes).to_string();
        Ok(OperationResult::new(output, "Binary decoded".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_encode() {
        let op = BinaryEncode;
        let result = op.perform("A").unwrap();
        assert_eq!(result.output, "01000001");
    }

    #[test]
    fn test_binary_decode() {
        let op = BinaryDecode;
        let result = op.perform("01000001").unwrap();
        assert_eq!(result.output, "A");
    }

    #[test]
    fn test_binary_roundtrip() {
        let encode = BinaryEncode;
        let decode = BinaryDecode;
        let input = "Hello!";
        let encoded = encode.perform(input).unwrap();
        let decoded = decode.perform(&encoded.output).unwrap();
        assert_eq!(decoded.output, input);
    }

    #[test]
    fn test_binary_decode_with_spaces() {
        let op = BinaryDecode;
        let result = op.perform("01000001 01000010").unwrap();
        assert_eq!(result.output, "AB");
    }

    #[test]
    fn test_binary_decode_invalid() {
        let op = BinaryDecode;
        let result = op.perform("010000010");
        assert!(result.is_err());
    }

    #[test]
    fn test_binary_decode_invalid_char() {
        let op = BinaryDecode;
        let result = op.perform("010000012");
        assert!(result.is_err());
    }
}
