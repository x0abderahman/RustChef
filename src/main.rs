use clap::Parser;
use std::io::Read;

use chef::cli::{Cli, Commands, execute_single, execute_pipeline, list_operations};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Op { operation, input, args } => {
            let input = get_input(input);
            if let Err(e) = execute_single(&operation, &input, &args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Run { pipeline, input, args } => {
            let input = get_input(input);
            if let Err(e) = execute_pipeline(&pipeline, &input, &args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::List => {
            list_operations();
        }
    }
}

/// Get input from command line argument or stdin
fn get_input(input: Option<String>) -> String {
    match input {
        Some(s) => s,
        None => {
            let mut buffer = String::new();
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            if handle.read_to_string(&mut buffer).is_err() {
                eprintln!("Warning: Could not read from stdin");
                String::new()
            } else {
                buffer.trim_end().to_string()
            }
        }
    }
}
