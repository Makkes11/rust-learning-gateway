use std::{fs::File, io::Error, num::ParseIntError};
fn main() {
    std::env::set_var("TEST_NUMBER", "99");
    if let Err(e) = "test".parse::<u32>() {
        print_user_message(e);
    }

    if let Err(err) = File::open("does_not_exist.txt") {
        print_user_message(err);
    }

    run_and_print(parse_and_validate::<u32, _>("12", |v| *v < 100));
    run_and_print(parse_and_validate::<usize, _>("abc", |_| true));
    run_and_print(parse_and_validate::<i32, _>("-5", |v| *v >= 0));

    read_from_source(ConstSource(42));

    read_from_source(EnvSource {
        var: "TEST_NUMBER".to_string(),
    });

    read_from_source(EnvSource {
        var: "MISSING_VAR".to_string(),
    });
}

// 3.2
trait ToUserMessage {
    fn to_user_message(&self) -> String;
}

impl ToUserMessage for ParseIntError {
    fn to_user_message(&self) -> String {
        self.to_string()
    }
}

impl ToUserMessage for Error {
    fn to_user_message(&self) -> String {
        self.to_string()
    }
}

fn print_user_message<E: ToUserMessage>(err: E) {
    println!("User message: {}", err.to_user_message());
}

// 3.3
fn parse_and_validate<T, F>(input: &str, validator: F) -> Result<T, String>
where
    T: std::str::FromStr,
    F: Fn(&T) -> bool,
{
    let parsed_input = input.parse::<T>();
    let input_result = match parsed_input {
        Err(_) => Err("Parse error".to_string()),
        Ok(res) => {
            if validator(&res) {
                Ok(res)
            } else {
                Err("Validation error".to_string())
            }
        }
    };

    input_result
}

fn run_and_print<T, E>(res: Result<T, E>)
where
    T: std::fmt::Debug,
    E: std::fmt::Display,
{
    match res {
        Ok(value) => println!("OK: {:?}", value),
        Err(err) => eprintln!("Fehler: {}", err),
    }
}

// 3.4
trait DataSource {
    fn read_value(&self) -> Result<u32, String>;
}

struct ConstSource(u32); // simple structure without anything, safes a u32 in itself

impl DataSource for ConstSource {
    fn read_value(&self) -> Result<u32, String> {
        Ok(self.0) // returns safed value -> tuple
    }
}

struct EnvSource {
    var: String,
}

impl DataSource for EnvSource {
    fn read_value(&self) -> Result<u32, String> {
        match std::env::var(&self.var) {
            Ok(value) => value.parse::<u32>().map_err(|_| "Env error".to_string()),
            Err(_) => Err("Env error".to_string()),
        }
    }
}

fn read_from_source<S: DataSource>(src: S) {
    run_and_print(src.read_value())
}
