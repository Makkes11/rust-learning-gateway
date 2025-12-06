use std::fs;

fn main() -> Result<(), ReadError> {
    match read_number("test.txt") {
        Ok(n) => println!("Number: {}", n),
        Err(e) => eprintln!("Error: {:?}", e), // hier wird der innerer Field gelesen
    }

    match safe_division(10, 0) {
        Ok(n) => println!("Number: {}", n),
        Err(e) => eprintln!("Error: {:?}", e), // hier wird der innerer Field gelesen
    }

    match safe_division(10, 2) {
        Ok(n) => println!("Number: {}", n),
        Err(e) => eprintln!("Error: {:?}", e), // hier wird der innerer Field gelesen
    }

    match load_and_double("test.txt") {
        Ok(n) => println!("Number: {}", n),
        Err(e) => eprintln!("Error: {:?}", e), // hier wird der innerer Field gelesen
    }

    Ok(())
}

fn read_number(path: &str) -> Result<i32, ReadError> {
    let content = fs::read_to_string(path)?;

    let first = content
        .split_whitespace()
        .next()
        .ok_or(ReadError::NoStringAfterSplit)?;

    first.trim().parse::<i32>().map_err(ReadError::Parse)
}

#[derive(Debug)]
enum ReadError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    NoStringAfterSplit,
}

impl From<std::io::Error> for ReadError {
    fn from(e: std::io::Error) -> Self {
        ReadError::Io(e)
    }
}

#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
}

fn safe_division(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }

    Ok(a / b)
}

#[derive(Debug)]
enum LoadError {
    Read(ReadError),
    Negative,
}

fn load_and_double(path: &str) -> Result<i32, LoadError> {
    let number = read_number(path).map_err(|e| LoadError::Read(e))?;
    if number < 0 {
        return Err(LoadError::Negative);
    }

    Ok(number * 2)
}
