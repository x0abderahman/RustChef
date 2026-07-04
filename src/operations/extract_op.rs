use super::{Operation, OperationResult};
use regex::Regex;

pub struct ExtractIPs;
pub struct ExtractURLs;
pub struct ExtractEmails;

/// Validate that a string is a proper IPv4 address (each octet <= 255)
fn is_valid_ip(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    parts.iter().all(|p| {
        p.parse::<u16>()
            .map(|n| n <= 255)
            .unwrap_or(false)
    })
}

impl Operation for ExtractIPs {
    fn name(&self) -> &'static str {
        "extract-ips"
    }

    fn description(&self) -> &'static str {
        "Extract valid IPv4 addresses from the input"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        let re = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b")
            .map_err(|e| format!("Regex error: {}", e))?;
        let ips: Vec<&str> = re
            .find_iter(input)
            .map(|m| m.as_str())
            .filter(|ip| is_valid_ip(ip))
            .collect();
        let output = ips.join("\n");
        let count = ips.len();
        Ok(OperationResult::new(
            output,
            format!("Extracted {} IP address(es)", count),
        ))
    }
}

impl Operation for ExtractURLs {
    fn name(&self) -> &'static str {
        "extract-urls"
    }

    fn description(&self) -> &'static str {
        "Extract URLs from the input"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        // Match common URL patterns and strip trailing punctuation
        let re = Regex::new(r#"https?://[^\s<>"']+|www\.[^\s<>"']+"#)
            .map_err(|e| format!("Regex error: {}", e))?;
        let urls: Vec<String> = re
            .find_iter(input)
            .map(|m| {
                let url = m.as_str();
                // Strip common trailing punctuation that's not part of URLs
                url.trim_end_matches(['.', ',', ';', ')', ']', '>'])
                    .to_string()
            })
            .filter(|url| !url.is_empty())
            .collect();
        let output = urls.join("\n");
        let count = urls.len();
        Ok(OperationResult::new(
            output,
            format!("Extracted {} URL(s)", count),
        ))
    }
}

impl Operation for ExtractEmails {
    fn name(&self) -> &'static str {
        "extract-emails"
    }

    fn description(&self) -> &'static str {
        "Extract email addresses from the input"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        // A simple but effective email regex
        let re = Regex::new(r#"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"#)
            .map_err(|e| format!("Regex error: {}", e))?;
        let emails: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
        let output = emails.join("\n");
        let count = emails.len();
        Ok(OperationResult::new(
            output,
            format!("Extracted {} email address(es)", count),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ips() {
        let op = ExtractIPs;
        let input = "Server 192.168.1.1 and 10.0.0.255 are active. Invalid: 999.999.999.999";
        let result = op.perform(input).unwrap();
        let ips: Vec<&str> = result.output.lines().collect();
        assert_eq!(ips, vec!["192.168.1.1", "10.0.0.255"]);
    }

    #[test]
    fn test_extract_ips_none() {
        let op = ExtractIPs;
        let result = op.perform("No IPs here").unwrap();
        assert_eq!(result.output, "");
    }

    #[test]
    fn test_extract_ips_valid_range() {
        let op = ExtractIPs;
        let input = "Valid: 0.0.0.0, 255.255.255.255, 192.168.0.1. Invalid: 256.1.1.1";
        let result = op.perform(input).unwrap();
        let ips: Vec<&str> = result.output.lines().collect();
        assert_eq!(ips, vec!["0.0.0.0", "255.255.255.255", "192.168.0.1"]);
    }

    #[test]
    fn test_extract_urls() {
        let op = ExtractURLs;
        let input = "Visit https://example.com/path or http://test.org. Also www.site.com";
        let result = op.perform(input).unwrap();
        let urls: Vec<&str> = result.output.lines().collect();
        assert_eq!(urls, vec!["https://example.com/path", "http://test.org", "www.site.com"]);
    }

    #[test]
    fn test_extract_emails() {
        let op = ExtractEmails;
        let input = "Contact support@example.com or admin@test.org. Invalid: @test.";
        let result = op.perform(input).unwrap();
        let emails: Vec<&str> = result.output.lines().collect();
        assert_eq!(emails, vec!["support@example.com", "admin@test.org"]);
    }

    #[test]
    fn test_extract_emails_none() {
        let op = ExtractEmails;
        let result = op.perform("No emails here").unwrap();
        assert_eq!(result.output, "");
    }
}
