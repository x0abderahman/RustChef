use super::{Operation, OperationResult};

pub struct TextStats;
pub struct Entropy;

impl Operation for TextStats {
    fn name(&self) -> &'static str {
        "text-stats"
    }

    fn description(&self) -> &'static str {
        "Compute text statistics (characters, words, lines, bytes)"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let char_count = input.chars().count();
        let word_count = if input.trim().is_empty() {
            0
        } else {
            input.split_whitespace().count()
        };
        let line_count = if input.is_empty() {
            0
        } else {
            input.lines().count()
        };
        let byte_count = input.len();

        let output = format!(
            "Characters: {}\nWords: {}\nLines: {}\nBytes: {}",
            char_count, word_count, line_count, byte_count
        );
        Ok(OperationResult::new(output, "Text statistics computed".to_string()))
    }
}

impl Operation for Entropy {
    fn name(&self) -> &'static str {
        "entropy"
    }

    fn description(&self) -> &'static str {
        "Estimate Shannon entropy of the input (in bits per byte)"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        if input.is_empty() {
            return Ok(OperationResult::new(
                "0.0".to_string(),
                "Entropy (bits per byte)".to_string(),
            ));
        }

        let bytes = input.as_bytes();
        let len = bytes.len() as f64;

        // Count byte frequencies
        let mut freq = [0u64; 256];
        for &b in bytes {
            freq[b as usize] += 1;
        }

        // Compute Shannon entropy
        let entropy: f64 = freq
            .iter()
            .filter(|&&c| c > 0)
            .map(|&c| {
                let p = c as f64 / len;
                -p * p.log2()
            })
            .sum();

        let output = format!("{:.4}", entropy);
        Ok(OperationResult::new(output, "Entropy (bits per byte)".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_stats_basic() {
        let op = TextStats;
        let result = op.perform("hello world").unwrap();
        assert!(result.output.contains("Characters: 11"));
        assert!(result.output.contains("Words: 2"));
        assert!(result.output.contains("Lines: 1"));
        assert!(result.output.contains("Bytes: 11"));
    }

    #[test]
    fn test_text_stats_empty() {
        let op = TextStats;
        let result = op.perform("").unwrap();
        assert!(result.output.contains("Characters: 0"));
        assert!(result.output.contains("Words: 0"));
        assert!(result.output.contains("Lines: 0"));
        assert!(result.output.contains("Bytes: 0"));
    }

    #[test]
    fn test_text_stats_multiline() {
        let op = TextStats;
        let result = op.perform("line1\nline2\nline3").unwrap();
        assert!(result.output.contains("Characters: 17"));
        assert!(result.output.contains("Lines: 3"));
    }

    #[test]
    fn test_entropy_uniform() {
        let op = Entropy;
        // All same character -> entropy should be 0
        let result = op.perform("AAAA").unwrap();
        let val: f64 = result.output.parse().unwrap();
        assert!((val - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_entropy_max() {
        let op = Entropy;
        // Two different bytes with equal probability -> entropy should be 1.0
        let result = op.perform("AB").unwrap();
        let val: f64 = result.output.parse().unwrap();
        assert!((val - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_entropy_empty() {
        let op = Entropy;
        let result = op.perform("").unwrap();
        assert_eq!(result.output, "0.0");
    }
}
