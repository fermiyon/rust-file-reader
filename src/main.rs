//! This module provides functionality for reading lines from a file specified by the user.
//!
//! The main function demonstrates how to open a file and read it line by line based on the user's input.
//! It accepts a command-line argument for the file path, handles errors related to file operations using match clause,
//! and reads each line of the file. If the file is not found, or another error occurs during file opening,
//! the program will panic with an appropriate error message. Similarly, if an error occurs while reading a line,
//! the program will panic.
//!
//! # Command-line Argument
//! `file_path`: The path to the file that will be read. If not specified, the program will panic.
//!
//! # Examples
//!
//! ```
//! // Run the program with the file path as an argument
//! cargo run -- example.txt
//! ```
//!
//! # Panics
//! The program panics if the file specified does not exist or cannot be read.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            panic!("No file path provided. Usage: cargo run -- <file_path>");
        }
    };

    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                panic!("File not found: {}", error);
            }
            _ => {
                panic!("Error opening file: {}", error);
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                panic!("Error reading line: {}", error);
            }
        };
        println!("{}", line);
    }
}
