use clap::Parser;
use shinkansen_lib::cli::Cli;

#[test]
fn test_cli_parsing() {
    // Test basic CLI parsing
    let args = vec!["shinkansen", "input.txt"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.inputs, vec!["input.txt"]);
    assert!(!cli.recursive);
    assert!(cli.output.is_none());
    assert!(cli.variables.is_empty());
    assert!(cli.config.is_none());
    assert!(cli.env.is_none());
}

#[test]
fn test_cli_with_all_options() {
    let args = vec![
        "shinkansen",
        "input.txt",
        "-r",
        "-o",
        "output.txt",
        "-D",
        "key=value",
        "-c",
        "config.json",
        "--env",
        "VAR1,VAR2",
    ];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.inputs, vec!["input.txt"]);
    assert!(cli.recursive);
    assert_eq!(cli.output, Some("output.txt".to_string()));
    assert_eq!(cli.variables, vec!["key=value"]);
    assert!(cli.config.is_some());
    assert_eq!(cli.env, Some("VAR1,VAR2".to_string()));
}

#[test]
fn test_cli_stdin() {
    let args = vec!["shinkansen", "-"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.inputs, vec!["-"]);
}

#[test]
fn test_cli_stdout() {
    let args = vec!["shinkansen", "input.txt", "-o", "-"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.output, Some("-".to_string()));
}

#[test]
fn test_cli_multiple_inputs() {
    let args = vec!["shinkansen", "file1.txt", "file2.txt", "file3.txt"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.inputs, vec!["file1.txt", "file2.txt", "file3.txt"]);
}

#[test]
fn test_cli_multiple_variables() {
    let args = vec![
        "shinkansen",
        "input.txt",
        "-D",
        "var1=value1",
        "-D",
        "var2=value2",
        "-D",
        "var3=value3",
    ];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(
        cli.variables,
        vec!["var1=value1", "var2=value2", "var3=value3"]
    );
}
