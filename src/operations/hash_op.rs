use super::{Operation, OperationResult};

pub struct MD5;
pub struct SHA1;
pub struct SHA256;

impl Operation for MD5 {
    fn name(&self) -> &'static str {
        "md5"
    }

    fn description(&self) -> &'static str {
        "Compute MD5 hash of the input"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        use md5::{Md5, Digest};
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        let output = format!("{:x}", result);
        Ok(OperationResult::new(output, "MD5 hash".to_string()))
    }
}

impl Operation for SHA1 {
    fn name(&self) -> &'static str {
        "sha1"
    }

    fn description(&self) -> &'static str {
        "Compute SHA-1 hash of the input"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        use sha1::Digest;
        let mut hasher = sha1::Sha1::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        let output = format!("{:x}", result);
        Ok(OperationResult::new(output, "SHA-1 hash".to_string()))
    }
}

impl Operation for SHA256 {
    fn name(&self) -> &'static str {
        "sha256"
    }

    fn description(&self) -> &'static str {
        "Compute SHA-256 hash of the input"
    }

    fn perform(&self, input: &str) -> Result<OperationResult, String> {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        let output = format!("{:x}", result);
        Ok(OperationResult::new(output, "SHA-256 hash".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let op = MD5;
        let result = op.perform("hello").unwrap();
        assert_eq!(result.output, "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_sha1() {
        let op = SHA1;
        let result = op.perform("hello").unwrap();
        assert_eq!(result.output, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    }

    #[test]
    fn test_sha256() {
        let op = SHA256;
        let result = op.perform("hello").unwrap();
        assert_eq!(
            result.output,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_md5_empty() {
        let op = MD5;
        let result = op.perform("").unwrap();
        assert_eq!(result.output, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_sha256_empty() {
        let op = SHA256;
        let result = op.perform("").unwrap();
        assert_eq!(
            result.output,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
