use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() {
    run_and_print(process_number_file("12", "ok.txt"));
    run_and_print(process_number_file("abc", "fail.txt"));
    run_and_print(process_number_file("-5", "neg.txt"));

    run_and_print(first_char_as_digit(""));

    run_and_print(first_char_as_digit("a45"));
    if let Ok(r) = first_char_as_digit("5xyz") {
        println!("{}", r)
    }
}

fn process_number_file(input: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let input_number: u32 = input.trim().parse()?;
    let output_number = input_number.pow(2);
    File::create(out_path)?.write_all(output_number.to_string().as_bytes())?;
    Ok(())
}

fn first_char_as_digit(input: &str) -> Result<u32, String> {
    let first = input.chars().next().ok_or("Empty string")?;

    let digit = first.to_digit(10).ok_or("Not a digit")?;

    Ok(digit)
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
