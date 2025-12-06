use std::{fs, io::Error};

use cli_advanced::*;

#[test]
fn integration_test_analyze_file_pass() -> Result<(), Error> {
    let path = "test_input.txt";
    fs::write(path, "Hello world\nLine 2")?;
    let result = analyze_file(path).unwrap();
    assert_eq!(result.lines, 2);
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
fn integration_config_from_args_error() -> Result<(), Error> {
    let args = vec![
        "program".to_string(),
        "input_real.txt".to_string(),
        // "lines".to_string(),
    ];

    //     Config::from_args_custom(args);
    let result = Config::from_args_custom(args);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn integration_config_from_args_pass() -> Result<(), Error> {
    let args = vec![
        "program".to_string(),
        "test_input.txt".to_string(),
        "lines".to_string(),
    ];

    //     Config::from_args_custom(args);
    let result = Config::from_args_custom(args);
    assert!(result.is_ok());
    let config = result.unwrap();

    assert_eq!(
        config,
        Config {
            filename: "test_input.txt".to_string(),
            mode: Mode::Lines
        }
    );

    Ok(())
}

#[test]
fn from_args_custom_missing_argument() {
    let args = vec!["only_one".to_string()];
    let err = Config::from_args_custom(args).expect_err("expected error");
    assert!(matches!(err, CliError::MissingArgument));
}
