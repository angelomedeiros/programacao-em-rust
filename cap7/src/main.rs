use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{stderr, ErrorKind, Read, Seek, SeekFrom, Write};
use std::{fmt, io};

fn main() {
    let err2 = MyError::CustomError("Simulação 2".to_string());

    print_error(&err2);

    // let greeting_file_result = File::open("hello.txt");

    // let mut greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("Problem opening the file: {:?}", error),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    let mut greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let mut contents = String::new();
    let _ = greeting_file.read_to_string(&mut contents);

    println!("{:?}", contents);

    let _ = File::open("hell.txt").expect("hello.txt should be included in this project");

    let _ = read_username_from_file();

    let _ = read_username_from_file_short();

    let file = write_username_from_file_short();

    match file {
        Ok(()) => println!("Ok!"),
        Err(error) => println!("Error: {}", error),
    }
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

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn write_username_from_file_short() -> Result<(), io::Error> {
    let mut username_file = OpenOptions::new()
        .write(true)
        .create(true)
        // .truncate(true)
        .open("hellos.txt")?;

    username_file.seek(SeekFrom::End(0))?;

    username_file.write_all(b"\n")?;

    username_file.write_all("Angelo Medeiros".as_bytes())?;
    Ok(())
}
