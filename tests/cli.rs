use assert_cmd::Command;
use std::process::Output;
use std::str::from_utf8;
use strip_ansi_escapes::strip;

fn run_grepnir_with_args(args: &[&str]) -> Output {
    let output = Command::cargo_bin("grepnir")
        .unwrap()
        .args(args)
        .output()
        .expect("Failed to execute command");

    let stdout = strip(&output.stdout);
    let stderr = strip(&output.stderr);

    Output {
        stdout,
        stderr,
        ..output
    }
}

#[test]
fn test_basic_pattern_matching() {
    let args = ["Alan Watts", "tests/inputs/phrase.txt"];

    let output = run_grepnir_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("Alan Watts"));
}

#[test]
fn test_case_insensitive_search() {
    let args = ["-i", "alan watts", "tests/inputs/phrase.txt"];

    let output = run_grepnir_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("Alan Watts"));
}

#[test]
fn test_recursive_search() {
    let args = ["-r", "they", "tests/inputs"];

    let output = run_grepnir_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("tests/inputs/tao.txt:Then they can be long-lived."));
}

#[test]
fn test_inverted_match() {
    let args = ["-v", "tests/inputs/phrase.txt", "Alan"];

    let output = run_grepnir_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(!stdout.contains("Alan"));
}
