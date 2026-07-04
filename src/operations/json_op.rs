use super::{Operation, OperationResult};
use serde_json::Value;

pub struct JsonPretty;
pub struct JsonMinify;

impl Operation for JsonPretty {
    fn name(&self) -> &'static str {
        "json-pretty"
    }

    fn description(&self) -> &'static str {
        "Pretty-print JSON data"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let value: Value = serde_json::from_str(input)
            .map_err(|e| format!("JSON parse error: {}", e))?;
        let output = serde_json::to_string_pretty(&value)
            .map_err(|e| format!("JSON serialize error: {}", e))?;
        Ok(OperationResult::new(output, "JSON pretty-printed".to_string()))
    }
}

impl Operation for JsonMinify {
    fn name(&self) -> &'static str {
        "json-minify"
    }

    fn description(&self) -> &'static str {
        "Minify JSON data (remove whitespace)"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let value: Value = serde_json::from_str(input)
            .map_err(|e| format!("JSON parse error: {}", e))?;
        let output = serde_json::to_string(&value)
            .map_err(|e| format!("JSON serialize error: {}", e))?;
        Ok(OperationResult::new(output, "JSON minified".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_pretty() {
        let op = JsonPretty;
        let input = r#"{"name":"test","value":123}"#;
        let result = op.perform(input).unwrap();
        assert!(result.output.contains("\n"));
        assert!(result.output.contains("  "));
    }

    #[test]
    fn test_json_minify() {
        let op = JsonMinify;
        let input = "{\n  \"name\": \"test\",\n  \"value\": 123\n}";
        let result = op.perform(input).unwrap();
        assert!(!result.output.contains("\n"));
        assert!(!result.output.contains("  "));
    }

    #[test]
    fn test_json_roundtrip() {
        let pretty = JsonPretty;
        let minify = JsonMinify;
        let input = r#"{"a":1,"b":2}"#;
        let pretty_output = pretty.perform(input).unwrap();
        let minify_output = minify.perform(&pretty_output.output).unwrap();
        // After minifying pretty output, we should get a compact representation
        let val1: serde_json::Value = serde_json::from_str(input).unwrap();
        let val2: serde_json::Value = serde_json::from_str(&minify_output.output).unwrap();
        assert_eq!(val1, val2);
    }

    #[test]
    fn test_json_invalid() {
        let op = JsonPretty;
        let result = op.perform("not json");
        assert!(result.is_err());
    }
}
