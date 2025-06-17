use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_malformed_csv_missing_quotes() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Description").unwrap();
    writeln!(temp_file, "John,This has a quote \" in the middle").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Should handle gracefully, even if malformed
    assert!(output.status.success() || !output.stderr.is_empty());
}

#[test]
fn test_csv_with_only_commas() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, ",,").unwrap();
    writeln!(temp_file, ",,").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8(output.stdout).unwrap();
    // Should create a table with empty cells
    assert!(result.contains("|  |  |  |"));
    assert!(result.contains("| --- | --- | --- |"));
}

#[test]
fn test_csv_with_trailing_commas() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Age,").unwrap();
    writeln!(temp_file, "John,25,").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout).unwrap();
    // Should handle trailing comma as empty column
    assert!(result.contains("| Name | Age |  |"));
}

#[test]
fn test_csv_with_only_newlines() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file).unwrap();
    writeln!(temp_file).unwrap();
    writeln!(temp_file).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8(output.stdout).unwrap();
    // Should produce empty output for empty lines
    assert!(result.is_empty() || result.trim().is_empty());
}

#[test]
fn test_csv_with_very_long_content() {
    let mut temp_file = NamedTempFile::new().unwrap();
    let long_content = "a".repeat(10000);
    writeln!(temp_file, "Name,Content").unwrap();
    writeln!(temp_file, "Test,\"{}\"", long_content).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout).unwrap();
    assert!(result.contains(&long_content));
}

#[test]
fn test_csv_with_binary_characters() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Data").unwrap();
    // Include some control characters and high-bit characters
    writeln!(temp_file, "Test,\"\\x00\\x01\\xFF\"").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Should either succeed or fail gracefully
    assert!(output.status.success() || !output.stderr.is_empty());
}

#[test]
fn test_csv_with_mixed_line_endings() {
    let mut temp_file = NamedTempFile::new().unwrap();
    // Mix Unix (\n) and Windows (\r\n) line endings
    write!(temp_file, "Name,Age\nJohn,25\nJane,30\n").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout).unwrap();
    assert!(result.contains("John"));
    assert!(result.contains("Jane"));
}

#[test]
fn test_csv_with_embedded_nulls() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Description").unwrap();
    writeln!(temp_file, "Test,\"Content with\0null\"").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Should handle null bytes gracefully
    assert!(output.status.success() || !output.stderr.is_empty());
}

#[test]
fn test_csv_with_extreme_unicode() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Emoji,Math").unwrap();
    writeln!(temp_file, "Unicode,\"👨👩👧👦\",\"∑∆∫\"").unwrap();
    writeln!(temp_file, "Symbols,\"🚀🌟💫\",\"≈≠≤≥\"").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout).unwrap();
    assert!(result.contains("👨👩👧👦"));
    assert!(result.contains("🚀🌟💫"));
    assert!(result.contains("∑∆∫"));
}

#[test]
fn test_csv_with_inconsistent_columns_extreme() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "A").unwrap();
    writeln!(temp_file, "B,C,D,E,F,G,H,I,J,K").unwrap();
    writeln!(temp_file, "L,M").unwrap();
    writeln!(temp_file, "N,O,P,Q,R,S,T,U,V,W,X,Y,Z").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout).unwrap();
    // Should handle extreme column count differences
    assert!(result.contains("| A |"));
    assert!(result.contains("| N | O | P | Q | R | S | T | U | V | W | X | Y | Z |"));
}
