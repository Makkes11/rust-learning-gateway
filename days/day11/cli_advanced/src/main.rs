use cli_advanced::{CliError, Config, analyze_file};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), CliError> {
    let config = Config::from_args()?;
    let result = analyze_file(&config.filename)?;

    match config.mode {
        cli_advanced::Mode::Lines => println!("{:?} {:?}", config.mode, result.lines),
        cli_advanced::Mode::Words => println!("{:?} {:?}", config.mode, result.words),
        cli_advanced::Mode::Chars => println!("{:?} {:?}", config.mode, result.chars),
        cli_advanced::Mode::All => println!("{:?} {:?}", config.mode, result),
    }

    Ok(())
}
