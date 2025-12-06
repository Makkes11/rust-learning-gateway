use std::{fs, str::FromStr};

#[derive(Debug)]
pub enum CliError {
    MissingArgument,
    Io(std::io::Error),
    InvalidMode,
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e)
    }
}

#[derive(Debug)]
pub struct Analysis {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

pub fn analyze_file(path: &str) -> Result<Analysis, CliError> {
    let content = fs::read_to_string(path)?;

    Ok(Analysis {
        lines: (content.lines().count()),
        words: (content.split_whitespace().count()),
        chars: (content.chars().count()),
    })
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Lines,
    Words,
    Chars,
    All,
}

impl FromStr for Mode {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lines" => Ok(Mode::Lines),
            "words" => Ok(Mode::Words),
            "chars" => Ok(Mode::Chars),
            "all" => Ok(Mode::All),
            _ => Err(CliError::InvalidMode),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub filename: String,
    pub mode: Mode,
}

impl Config {
    fn from_vec(args: Vec<String>) -> Result<Self, CliError> {
        if args.len() != 3 {
            return Err(CliError::MissingArgument);
        }

        let filename = args[1].clone();
        let mode = args[2].parse::<Mode>()?;

        Ok(Config { filename, mode })
    }

    pub fn from_args() -> Result<Self, CliError> {
        Self::from_vec(std::env::args().collect())
    }

    pub fn from_args_custom(args: Vec<String>) -> Result<Self, CliError> {
        Self::from_vec(args)
    }
}
