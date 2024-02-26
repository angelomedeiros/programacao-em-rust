use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{stderr, Write};

fn main() {
    let err2 = MyError::CustomError("Simulação 2".to_string());

    print_error(&err2);

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}!!!", err);

    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

#[derive(Debug)]
enum MyError {
    CustomError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::CustomError(msg) => write!(f, "Custom Error: {}", msg),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::CustomError(_) => None,
        }
    }
}
