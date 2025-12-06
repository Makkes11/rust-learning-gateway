use cli_commands::{CliError, Config};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), CliError> {
    let config = Config::from_args()?;
    let output = cli_commands::execute(config.command)?;

    println!("{}", output);

    Ok(())
}
