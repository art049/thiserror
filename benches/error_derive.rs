use std::fmt;
use std::io;
use thiserror::Error;

fn main() {
    divan::main();
}

// Define various error types for benchmarking

#[derive(Error, Debug)]
#[error("simple unit error")]
struct SimpleError;

#[derive(Error, Debug)]
#[error("tuple error: {0}")]
struct TupleError(String);

#[derive(Error, Debug)]
#[error("struct error: {message}")]
struct StructError {
    message: String,
}

#[derive(Error, Debug)]
enum ComplexError {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("invalid value: expected {expected}, got {actual}")]
    InvalidValue { expected: i32, actual: i32 },
}

#[derive(Error, Debug)]
#[error("transparent error")]
struct TransparentError(#[from] io::Error);

// Benchmarks

#[divan::bench]
fn create_simple_error() -> SimpleError {
    SimpleError
}

#[divan::bench]
fn create_tuple_error() -> TupleError {
    TupleError("benchmark error message".to_string())
}

#[divan::bench]
fn create_struct_error() -> StructError {
    StructError {
        message: "benchmark error message".to_string(),
    }
}

#[divan::bench]
fn create_complex_enum_error() -> ComplexError {
    ComplexError::Parse("invalid input".to_string())
}

#[divan::bench]
fn create_complex_struct_variant() -> ComplexError {
    ComplexError::InvalidValue {
        expected: 42,
        actual: 0,
    }
}

#[divan::bench]
fn format_simple_error() -> String {
    let error = SimpleError;
    format!("{}", error)
}

#[divan::bench]
fn format_tuple_error() -> String {
    let error = TupleError("benchmark error message".to_string());
    format!("{}", error)
}

#[divan::bench]
fn format_struct_error() -> String {
    let error = StructError {
        message: "benchmark error message".to_string(),
    };
    format!("{}", error)
}

#[divan::bench]
fn format_complex_error() -> String {
    let error = ComplexError::InvalidValue {
        expected: 42,
        actual: 0,
    };
    format!("{}", error)
}

#[divan::bench]
fn error_source_chain() -> Option<String> {
    use std::error::Error;
    
    let io_error = io::Error::new(io::ErrorKind::Other, "underlying error");
    let error = ComplexError::Io(io_error);
    
    error.source().map(|s| s.to_string())
}

#[divan::bench]
fn debug_format_error() -> String {
    let error = ComplexError::InvalidValue {
        expected: 42,
        actual: 0,
    };
    format!("{:?}", error)
}

#[divan::bench(args = [10, 100, 1000])]
fn create_errors_in_loop(n: usize) -> Vec<ComplexError> {
    (0..n)
        .map(|i| ComplexError::InvalidValue {
            expected: 42,
            actual: i as i32,
        })
        .collect()
}

#[divan::bench(args = [10, 100, 1000])]
fn format_errors_in_loop(n: usize) -> Vec<String> {
    let errors: Vec<_> = (0..n)
        .map(|i| ComplexError::InvalidValue {
            expected: 42,
            actual: i as i32,
        })
        .collect();
    
    errors.iter().map(|e| format!("{}", e)).collect()
}
