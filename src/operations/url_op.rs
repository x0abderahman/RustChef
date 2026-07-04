use super::{Operation, OperationResult};

pub struct UrlEncode;
pub struct UrlDecode;

impl Operation for UrlEncode {
    fn name(&self) -> &'static str {
        "url-encode"
    }

    fn description(&self) -> &'static str {
        "URL encode input data"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let encoded = urlencoding::encode(input);
        Ok(OperationResult::new(
            encoded.into_owned(),
            "URL encoded".to_string(),
        ))
    }
}

impl Operation for UrlDecode {
    fn name(&self) -> &'static str {
        "url-decode"
    }

    fn description(&self) -> &'static str {
        "URL decode input data"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let trimmed = input.trim();
        let decoded = urlencoding::decode(trimmed)
            .map_err(|e| format!("URL decode error: {}", e))?;
        Ok(OperationResult::new(
            decoded.into_owned(),
            "URL decoded".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        let op = UrlEncode;
        let result = op.perform("hello world").unwrap();
        assert_eq!(result.output, "hello%20world");
    }

    #[test]
    fn test_url_decode() {
        let op = UrlDecode;
        let result = op.perform("hello%20world").unwrap();
        assert_eq!(result.output, "hello world");
    }

    #[test]
    fn test_url_encode_special_chars() {
        let op = UrlEncode;
        let result = op.perform("a&b=c+d").unwrap();
        assert_eq!(result.output, "a%26b%3Dc%2Bd");
    }

    #[test]
    fn test_url_decode_special_chars() {
        let op = UrlDecode;
        let result = op.perform("a%26b%3Dc%2Bd").unwrap();
        assert_eq!(result.output, "a&b=c+d");
    }

    #[test]
    fn test_url_roundtrip() {
        let encode = UrlEncode;
        let decode = UrlDecode;
        let input = "Hello, World! foo@bar.com?a=1&b=2";
        let encoded = encode.perform(input).unwrap();
        let decoded = decode.perform(&encoded.output).unwrap();
        assert_eq!(decoded.output, input);
    }
}
