use std::{fs, io::Error};

use cli_commands::*;

#[test]
fn integration_test_analyze_file_pass() -> Result<(), Error> {
    let path = "test_input.txt";
    fs::write(path, "Hello world\nLine 2")?;
    let result = analyze_file(path).unwrap();
    assert_eq!(result.lines, 2);
    Ok(())
}

#[test]
fn integration_test_analyze_emptys_file_pass() -> Result<(), Error> {
    let path = "empty.txt";
    fs::write(path, "")?; // wichtig: Datei erzeugen
    let result = analyze_file(path).unwrap();
    assert_eq!(result.lines, 0);
    Ok(())
}

#[test]
fn integration_test_analyze_file_fail() -> Result<(), Error> {
    let path = "test.txt";
    let result = analyze_file(path);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn from_args_custom_missing_argument() {
    let args = vec!["only_one".to_string()];
    let err = Config::from_args_custom(args).expect_err("expected error");
    assert!(matches!(err, CliError::MissingArgument));
}

#[test]
fn parse_count_command() {
    let args: Vec<String> = vec!["mycli".into(), "count".into(), "test_input.txt".into()];
    let cmd = parse_args(&args).unwrap();
    assert_eq!(
        cmd,
        Command::Count {
            filename: "test_input.txt".into()
        }
    );
}

#[test]
fn parse_count_command_fails() {
    let args: Vec<String> = vec!["mycli".into(), "what".into(), "test_input.txt".into()];
    let cmd = parse_args(&args);
    assert!(cmd.is_err());
}

#[test]
fn parse_echo_command() {
    let args: Vec<String> = vec!["mycli".into(), "echo".into(), "what is this?".into()];
    let cmd = parse_args(&args).unwrap();
    assert_eq!(
        cmd,
        Command::Echo {
            text: "what is this?".into()
        }
    );
}

#[test]
fn parse_yaml_command() {
    let args: Vec<String> = vec!["mycli".into(), "statsyaml".into(), "test.txt".into()];
    let cmd = parse_args(&args).unwrap();
    assert_eq!(
        cmd,
        Command::StatsYAML {
            filename: "test.txt".into()
        }
    );
}

#[test]
fn parse_size_command() {
    let args: Vec<String> = vec!["mycli".into(), "size".into(), "test.txt".into()];
    let cmd = parse_args(&args).unwrap();
    assert_eq!(
        cmd,
        Command::FileSize {
            filename: "test.txt".into()
        }
    );
}

#[test]
fn parse_help_command() {
    let args: Vec<String> = vec!["mycli".into(), "--help".into()];
    let config = Config::from_args_custom(args).unwrap();
    assert_eq!(config.command, Command::Help);
}

#[test]
fn parse_help_command_fails() {
    let args: Vec<String> = vec!["mycli".into(), "--help".into(), "something".into()];
    let err = parse_args(&args).unwrap_err();
    assert!(matches!(err, CliError::MissingArgument));
}

#[test]
fn parse_command_fails() {
    let args: Vec<String> = vec!["mycli".into()];
    let err = Config::from_args_custom(args);
    assert!(err.is_err())
}

#[test]
fn parse_command_fails_count_no_arg() {
    let args: Vec<String> = vec!["mycli".into(), "count".into()];
    let err = Config::from_args_custom(args).unwrap_err();
    assert!(matches!(err, CliError::MissingArgument));
}

#[test]
fn analyze_file_unreadable() -> std::io::Result<()> {
    let bad_bytes = [0x80];
    let path = "fail.txt";
    fs::write(path, &bad_bytes)?;
    let err = analyze_file(path);

    assert!(err.is_err());
    Ok(())
}

#[test]
fn get_file_size_byte() -> std::io::Result<()> {
    let string = "test";
    let path = "size.txt";
    fs::write(path, &string)?;
    let result = get_file_size(path).unwrap();

    assert_eq!(result, 4);
    Ok(())
}

#[test]
fn get_file_size_fails_no_file() -> std::io::Result<()> {
    let err = get_file_size("nopath.txt").unwrap_err();

    assert!(matches!(err, CliError::Io(_)));
    Ok(())
}
