use clap::{Parser, Subcommand};

use crate::operations::base64_op::{Base64Encode, Base64Decode};
use crate::operations::hex_op::{HexEncode, HexDecode};
use crate::operations::url_op::{UrlEncode, UrlDecode};
use crate::operations::rot13_op::ROT13;
use crate::operations::xor_op::XOR;
use crate::operations::hash_op::{MD5, SHA1, SHA256};
use crate::operations::extract_op::{ExtractIPs, ExtractURLs, ExtractEmails};
use crate::operations::text_op::{TextStats, Entropy};
use crate::operations::binary_op::{BinaryEncode, BinaryDecode};
use crate::operations::json_op::{JsonPretty, JsonMinify};
use crate::operations::case_op::{LowerCase, UpperCase, Reverse};
use crate::operations::Operation;
use crate::pipeline::Pipeline;

/// chef - A CyberChef-inspired CLI tool for data transformation
///
/// Encode, decode, hash, extract, and transform data from the command line.
/// Operations can be chained together using the 'run' command.
#[derive(Parser)]
#[command(name = "chef")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a single operation on input
    Op {
        /// The operation to perform
        operation: String,

        /// Input text (or use stdin)
        input: Option<String>,

        /// Additional arguments for the operation (e.g., XOR key).
        /// Use '--' before the arguments, e.g.: chef op xor hello -- mykey
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// Run a pipeline of operations (comma-separated)
    Run {
        /// Comma-separated list of operations to chain
        pipeline: String,

        /// Input text (or use stdin)
        input: Option<String>,

        /// Additional arguments for operations
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// List all available operations
    List,
}

/// Get all available operation names
pub fn get_operation_names() -> Vec<&'static str> {
    vec![
        "base64-encode", "base64-decode",
        "hex-encode", "hex-decode",
        "url-encode", "url-decode",
        "rot13",
        "xor",
        "md5", "sha1", "sha256",
        "extract-ips", "extract-urls", "extract-emails",
        "text-stats", "entropy",
        "binary-encode", "binary-decode",
        "json-pretty", "json-minify",
        "lower-case", "upper-case", "reverse",
    ]
}

/// Create operation instances from a list of names, with optional arguments
pub fn create_operations(names: &[&str], args: &[String]) -> Result<Vec<Box<dyn Operation>>, String> {
    let mut ops: Vec<Box<dyn Operation>> = Vec::new();
    for &name in names {
        let op = create_single_operation(name, args)?;
        ops.push(op);
    }
    Ok(ops)
}

/// Create a single operation instance by name
pub fn create_single_operation(name: &str, args: &[String]) -> Result<Box<dyn Operation>, String> {
    match name {
        "base64-encode" => Ok(Box::new(Base64Encode)),
        "base64-decode" => Ok(Box::new(Base64Decode)),
        "hex-encode" => Ok(Box::new(HexEncode)),
        "hex-decode" => Ok(Box::new(HexDecode)),
        "url-encode" => Ok(Box::new(UrlEncode)),
        "url-decode" => Ok(Box::new(UrlDecode)),
        "rot13" => Ok(Box::new(ROT13)),
        "xor" => {
            let key = args.first().ok_or_else(|| {
                "XOR operation requires a key argument (use -- key-value)".to_string()
            })?;
            Ok(Box::new(XOR { key: key.clone() }))
        }
        "md5" => Ok(Box::new(MD5)),
        "sha1" => Ok(Box::new(SHA1)),
        "sha256" => Ok(Box::new(SHA256)),
        "extract-ips" => Ok(Box::new(ExtractIPs)),
        "extract-urls" => Ok(Box::new(ExtractURLs)),
        "extract-emails" => Ok(Box::new(ExtractEmails)),
        "text-stats" => Ok(Box::new(TextStats)),
        "entropy" => Ok(Box::new(Entropy)),
        "binary-encode" => Ok(Box::new(BinaryEncode)),
        "binary-decode" => Ok(Box::new(BinaryDecode)),
        "json-pretty" => Ok(Box::new(JsonPretty)),
        "json-minify" => Ok(Box::new(JsonMinify)),
        "lower-case" => Ok(Box::new(LowerCase)),
        "upper-case" => Ok(Box::new(UpperCase)),
        "reverse" => Ok(Box::new(Reverse)),
        _ => Err(format!("Unknown operation: {}", name)),
    }
}

/// Validate that all operation names in a pipeline are known
pub fn validate_pipeline_operations(pipeline_str: &str) -> Result<Vec<String>, String> {
    let names: Vec<String> = pipeline_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if names.is_empty() {
        return Err("Pipeline must contain at least one operation".to_string());
    }

    let valid_ops = get_operation_names();
    for name in &names {
        if !valid_ops.contains(&name.as_str()) {
            return Err(format!(
                "Unknown operation '{}' in pipeline. Available operations: {}",
                name,
                valid_ops.join(", ")
            ));
        }
    }
    Ok(names)
}

/// Execute a single operation on input and print result
pub fn execute_single(op_name: &str, input: &str, args: &[String]) -> Result<(), String> {
    // Validate operation name first
    let valid_ops = get_operation_names();
    if !valid_ops.contains(&op_name) {
        return Err(format!(
            "Unknown operation '{}'. Available operations: {}",
            op_name,
            valid_ops.join(", ")
        ));
    }

    let op = create_single_operation(op_name, args)?;
    let result = op.perform(input)?;
    println!("{}", result.output);
    eprintln!("// {}: {}", op.name(), result.description);
    Ok(())
}

/// Execute a pipeline of operations on input and print results
pub fn execute_pipeline(pipeline_str: &str, input: &str, args: &[String]) -> Result<(), String> {
    let names = validate_pipeline_operations(pipeline_str)?;
    let ops = create_operations(&names.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), args)?;

    let mut pipe = Pipeline::new();
    for op in ops {
        pipe.add(op);
    }
    let results = pipe.run(input)?;
    if let Some(last) = results.last() {
        println!("{}", last.output);
    }
    for (i, result) in results.iter().enumerate() {
        eprintln!("// Step {}: {}", i + 1, result.description);
    }
    Ok(())
}

/// List all available operations
pub fn list_operations() {
    let ops = get_operation_names();
    println!("Available operations ({}):", ops.len());
    for op_name in &ops {
        if let Ok(instance) = create_single_operation(op_name, &[]) {
            println!("  {}  : {}", op_name, instance.description());
        } else {
            println!("  {}  : (requires arguments)", op_name);
        }
    }
    println!();
    println!("Usage:");
    println!("  chef op <operation> [input]           Run a single operation");
    println!("  chef run <op1,op2,...> [input]        Run a pipeline of operations");
    println!("  chef list                             List all operations");
    println!();
    println!("For operations requiring arguments (like xor), use '--' followed by args:");
    println!("  chef op xor \"hello\" -- mykey");
    println!("  chef run \"base64-encode,hex-encode\" \"hello\"");
    println!("If no input is provided, reads from stdin.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_operation_names() {
        let names = get_operation_names();
        assert!(names.len() >= 15);
        assert!(names.contains(&"base64-encode"));
        assert!(names.contains(&"rot13"));
        assert!(names.contains(&"sha256"));
    }

    #[test]
    fn test_create_single_operation_valid() {
        let op = create_single_operation("base64-encode", &[]);
        assert!(op.is_ok());
    }

    #[test]
    fn test_create_single_operation_invalid() {
        let op = create_single_operation("nonexistent", &[]);
        assert!(op.is_err());
    }

    #[test]
    fn test_create_xor_without_key() {
        let op = create_single_operation("xor", &[]);
        assert!(op.is_err());
    }

    #[test]
    fn test_create_xor_with_key() {
        let op = create_single_operation("xor", &["key123".to_string()]);
        assert!(op.is_ok());
    }

    #[test]
    fn test_validate_pipeline() {
        let result = validate_pipeline_operations("base64-encode,hex-encode");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["base64-encode", "hex-encode"]);
    }

    #[test]
    fn test_validate_pipeline_invalid() {
        let result = validate_pipeline_operations("base64-encode,unknown");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_pipeline_empty() {
        let result = validate_pipeline_operations("");
        assert!(result.is_err());
    }
}
