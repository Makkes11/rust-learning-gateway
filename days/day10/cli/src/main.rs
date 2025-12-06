use std::{env, fs};

fn main() -> std::io::Result<()> {
    let filename = match get_filename() {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Usage: wc-light <filename>");
            std::process::exit(1);
        }
    };
    demo_io()?;
    wc(&filename)?;
    Ok(())
}

fn get_filename() -> Result<String, std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Usage: wc-light <filename>",
        ));
    }

    Ok(args[1].clone())
}

fn demo_io() -> Result<String, std::io::Error> {
    // Demo 1: Datei lesen
    let content = fs::read_to_string("input.txt")?;
    println!("Content:\n{content}");

    // Demo 2: Datei schreiben
    fs::write("output.txt", "Ich wurde von Rust erstellt!\n")?;
    Ok(content)
}

fn wc(filename: &str) -> std::io::Result<()> {
    let content = fs::read_to_string(filename)?;
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();

    println!("Lines: {lines}");
    println!("Words: {words}");
    println!("Chars: {chars}");

    Ok(())
}
