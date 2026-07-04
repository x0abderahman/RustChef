use super::{Operation, OperationResult};

pub struct HexEncode;
pub struct HexDecode;

impl Operation for HexEncode {
    fn name(&self) -> &'static str {
        "hex-encode"
    }

    fn description(&self) -> &'static str {
        "Encode input data to hexadecimal"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let encoded = hex::encode(input.as_bytes());
        Ok(OperationResult::new(encoded, "Hex encoded".to_string()))
    }
}

impl Operation for HexDecode {
    fn name(&self) -> &'static str {
        "hex-decode"
    }

    fn description(&self) -> &'static str {
        "Decode hexadecimal data"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let trimmed = input.trim();
        let decoded = hex::decode(trimmed)
            .map_err(|e| format!("Hex decode error: {}", e))?;
        let output = String::from_utf8_lossy(&decoded).to_string();
        Ok(OperationResult::new(output, "Hex decoded".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_encode() {
        let op = HexEncode;
        let result = op.perform("hello").unwrap();
        assert_eq!(result.output, "68656c6c6f");
    }

    #[test]
    fn test_hex_decode() {
        let op = HexDecode;
        let result = op.perform("68656c6c6f").unwrap();
        assert_eq!(result.output, "hello");
    }

    #[test]
    fn test_hex_decode_uppercase() {
        let op = HexDecode;
        let result = op.perform("68656C6C6F").unwrap();
        assert_eq!(result.output, "hello");
    }

    #[test]
    fn test_hex_decode_invalid() {
        let op = HexDecode;
        let result = op.perform("xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_hex_roundtrip() {
        let encode = HexEncode;
        let decode = HexDecode;
        let input = "Hello, World!";
        let encoded = encode.perform(input).unwrap();
        let decoded = decode.perform(&encoded.output).unwrap();
        assert_eq!(decoded.output, input);
    }
}
