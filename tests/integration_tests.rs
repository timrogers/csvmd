use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_cli_with_file_input() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Age,City").unwrap();
    writeln!(temp_file, "John,25,NYC").unwrap();
    writeln!(temp_file, "Jane,30,LA").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8(output.stdout).unwrap();
    insta::assert_snapshot!(result);
}

#[test]
fn test_cli_with_stdin() {
    let csv_data = "Product,Price\nLaptop,$999\nMouse,$25";

    let output = Command::new("cargo")
        .args(["run"])
        .arg("--")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    output
        .stdin
        .as_ref()
        .unwrap()
        .write_all(csv_data.as_bytes())
        .unwrap();
    let result = output.wait_with_output().unwrap();

    let stdout = String::from_utf8(result.stdout).unwrap();
    insta::assert_snapshot!(stdout);
}

#[test]
fn test_cli_with_complex_csv() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Description,Tags").unwrap();
    writeln!(
        temp_file,
        "John,\"A person with\nmultiple lines\",\"tag1,tag2\""
    )
    .unwrap();
    writeln!(temp_file, "Jane,\"Has | pipes\",simple").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8(output.stdout).unwrap();
    insta::assert_snapshot!(result);
}

#[test]
fn test_cli_with_empty_file() {
    let temp_file = NamedTempFile::new().unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8(output.stdout).unwrap();
    insta::assert_snapshot!(result);
}

#[test]
fn test_cli_with_single_column() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Item").unwrap();
    writeln!(temp_file, "Apple").unwrap();
    writeln!(temp_file, "Banana").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8(output.stdout).unwrap();
    insta::assert_snapshot!(result);
}

#[test]
fn test_cli_help_flag() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8(output.stdout).unwrap();
    // Normalize output for cross-platform compatibility (remove .exe extension on Windows)
    let normalized_result = result.replace("csvmd.exe", "csvmd");
    insta::assert_snapshot!(normalized_result);
}

#[test]
fn test_cli_nonexistent_file() {
    let output = Command::new("cargo")
        .args(["run", "--", "/nonexistent/file.csv"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("No such file") || stderr.contains("cannot find"));
}

#[test]
fn test_cli_with_unicode() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Symbol,Name,Code").unwrap();
    writeln!(temp_file, "★,Star,U+2605").unwrap();
    writeln!(temp_file, "♠,Spade,U+2660").unwrap();
    writeln!(temp_file, "🚀,Rocket,U+1F680").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8(output.stdout).unwrap();
    insta::assert_snapshot!(result);
}

#[test]
fn test_cli_with_mixed_quote_styles() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Quote").unwrap();
    writeln!(temp_file, "Shakespeare,\"To be or not to be\"").unwrap();
    writeln!(temp_file, "Einstein,\"E=mc²\"").unwrap();
    writeln!(temp_file, "Anonymous,No quotes here").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8(output.stdout).unwrap();
    insta::assert_snapshot!(result);
}

#[test]
fn test_cli_stdin_with_piped_input_is_fast() {
    // Test that piped input doesn't have a 2-second delay
    let csv_data = "Product,Price\nLaptop,$999\nMouse,$25";

    let start = std::time::Instant::now();

    let mut child = Command::new("cargo")
        .args(["run"])
        .arg("--")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(csv_data.as_bytes())
        .unwrap();

    let result = child.wait_with_output().unwrap();
    let elapsed = start.elapsed();

    let stdout = String::from_utf8(result.stdout).unwrap();
    let expected = "| Product | Price |\n| --- | --- |\n| Laptop | $999 |\n| Mouse | $25 |\n";
    assert_eq!(stdout, expected);

    // Should complete quickly since input is piped (not interactive)
    assert!(
        elapsed.as_secs() < 2,
        "Piped input should not have 2-second delay, took {:?}",
        elapsed
    );
}
