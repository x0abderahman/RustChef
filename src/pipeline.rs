use crate::operations::Operation;
use crate::operations::OperationResult;

/// A pipeline that chains multiple operations together
pub struct Pipeline {
    operations: Vec<Box<dyn Operation>>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn add(&mut self, op: Box<dyn Operation>) {
        self.operations.push(op);
    }

    pub fn run(&self, input: &str) -> Result<Vec<OperationResult>, String> {
        let mut results = Vec::new();
        let mut current_input = input.to_string();

        for op in &self.operations {
            let result = op.perform(&current_input)?;
            current_input = result.output.clone();
            results.push(result);
        }

        Ok(results)
    }

    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    pub fn len(&self) -> usize {
        self.operations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::base64_op::Base64Encode;
    use crate::operations::case_op::Reverse;
    use crate::operations::hex_op::HexEncode;

    #[test]
    fn test_pipeline_single() {
        let mut pipe = Pipeline::new();
        pipe.add(Box::new(HexEncode));
        let results = pipe.run("hello").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].output, "68656c6c6f");
    }

    #[test]
    fn test_pipeline_chain() {
        let mut pipe = Pipeline::new();
        pipe.add(Box::new(HexEncode));
        pipe.add(Box::new(Reverse));
        let results = pipe.run("hello").unwrap();
        assert_eq!(results.len(), 2);
        // hex: "hello" -> "68656c6c6f", then reverse: "f6c6c65686"
        assert_eq!(results[0].output, "68656c6c6f");
        assert_eq!(results[1].output, "f6c6c65686");
    }

    #[test]
    fn test_pipeline_empty() {
        let pipe = Pipeline::new();
        assert!(pipe.is_empty());
    }

    #[test]
    fn test_pipeline_base64_then_hex() {
        let mut pipe = Pipeline::new();
        pipe.add(Box::new(Base64Encode));
        pipe.add(Box::new(HexEncode));
        // base64("hello") = "aGVsbG8=", hex("aGVsbG8=") = "614756736247383d"
        let results = pipe.run("hello").unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].output, "aGVsbG8=");
        assert_eq!(results[1].output, "614756736247383d");
    }
}
