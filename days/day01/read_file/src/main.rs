use std::{fs::File, io::Read, num::IntErrorKind};

fn main() {
    let file_path = "/home/dev/programming/rust/rust_book/9/read_file/src/example.txt";

    match read_file(file_path) {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Fehler beim Lesen der Datei: {}", e),
    }

    let test_number = "abc";
    match parse_positive_number(test_number) {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?; // Fehler wird direkt weitergereicht
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_positive_number(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();

    if trimmed.starts_with('-') {
        return Err("Negative number not allowed".to_string());
    }

    trimmed.parse::<u32>().map_err(|e| match e.kind() {
        IntErrorKind::InvalidDigit => "Not a number".to_string(),
        IntErrorKind::PosOverflow => "Number too large".to_string(),
        _ => "Unknown numeric error".to_string(),
    })
}
