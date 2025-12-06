use std::fs;

#[derive(Debug, PartialEq)]
pub enum Command {
    Count { filename: String },
    StatsJSON { filename: String },
    FileSize { filename: String },
    StatsYAML { filename: String },
    Echo { text: String },
    Help,
}

#[derive(Debug)]
pub enum CliError {
    MissingArgument,
    Io(std::io::Error),
    InvalidCommand,
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
pub struct Config {
    pub command: Command,
}

impl Config {
    fn from_vec(args: Vec<String>) -> Result<Self, CliError> {
        let command = parse_args(&args)?;
        Ok(Config { command: command })
    }

    pub fn from_args() -> Result<Self, CliError> {
        Self::from_vec(std::env::args().collect())
    }

    pub fn from_args_custom(args: Vec<String>) -> Result<Self, CliError> {
        Self::from_vec(args)
    }
}

pub fn parse_args(args: &[String]) -> Result<Command, CliError> {
    if args.len() <= 1 {
        return Err(CliError::MissingArgument);
    }

    if args[1] == "--help" && args.len() <= 2 {
        return Ok(Command::Help);
    }

    if args[1] == "--help" && args.len() > 2 {
        return Err(CliError::MissingArgument);
    }

    if args.len() != 3 {
        return Err(CliError::MissingArgument);
    }

    let command: &String = &args[1];
    match command.to_lowercase().as_str() {
        "count" => Ok(Command::Count {
            filename: (args[2].to_string()),
        }),
        "size" => Ok(Command::FileSize {
            filename: (args[2].to_string()),
        }),
        "statsjson" => Ok(Command::StatsJSON {
            filename: (args[2].to_string()),
        }),
        "statsyaml" => Ok(Command::StatsYAML {
            filename: (args[2].to_string()),
        }),
        "echo" => Ok(Command::Echo {
            text: (args[2].to_string()),
        }),

        _ => Err(CliError::InvalidCommand),
    }
}

pub fn execute(cmd: Command) -> Result<String, CliError> {
    match cmd {
        Command::Count { filename } => {
            let data = analyze_file(filename.as_str())?;
            Ok(format!(
                "lines: {}, words: {}, chars: {}",
                &data.lines, &data.words, &data.chars
            ))
        }
        Command::StatsJSON { filename } => {
            let data = analyze_file(filename.as_str())?;
            Ok(format!(
                r#"{{"lines": {}, "words": {}, "chars": {}}}"#,
                &data.lines, &data.words, &data.chars
            ))
        }
        Command::Echo { text } => Ok(format!("Echo: {}", text)),
        Command::FileSize { filename } => {
            let size = get_file_size(filename.as_str())?;
            Ok(format!("Size of file is: {size} Byte",))
        }
        Command::StatsYAML { filename } => {
            let data = analyze_file(filename.as_str())?;
            Ok(format!(
                "items:\n  lines: {}\n  words: {}\n  chars: {}",
                data.lines, &data.words, &data.chars
            ))
        }
        Command::Help => Ok(format_help()),
    }
}

fn format_help() -> String {
    format!(
        "Commands\n\t
        count <file>\tprints line/word/char counts\n\t
        size  <file> \tprints size of file\n\t
        statsjson <file>\tprints JSON analysis\n\t
        statsyaml <file>\tprints YAML analysis\n\t
        echo  <text>\tprints the text\n\t"
    )
}

pub fn get_file_size(path: &str) -> Result<u64, CliError> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}
