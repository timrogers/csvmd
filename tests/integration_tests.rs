use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

/// Comprehensive CLI output for snapshot testing
#[derive(Debug)]
struct CliOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl CliOutput {
    fn from_command_output(output: std::process::Output) -> Self {
        Self {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        }
    }

    /// Normalize cross-platform differences for consistent snapshots
    fn normalize(mut self) -> Self {
        // Remove .exe extension on Windows for cross-platform compatibility
        self.stdout = self.stdout.replace("csvmd.exe", "csvmd");
        self.stderr = self.stderr.replace("csvmd.exe", "csvmd");

        // Remove cargo build output from stderr to focus on application errors
        self.stderr = self
            .stderr
            .lines()
            .filter(|line| {
                !line.contains("Finished")
                    && !line.contains("Running")
                    && !line.trim_start().starts_with('\x1b') // Remove ANSI escape sequences
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();

        self
    }
}

/// Run csvmd command and capture comprehensive output
fn run_csvmd_with_args(args: &[&str]) -> CliOutput {
    let mut cmd_args = vec!["run", "--"];
    cmd_args.extend_from_slice(args);

    let output = Command::new("cargo")
        .args(&cmd_args)
        .output()
        .expect("Failed to execute command");

    CliOutput::from_command_output(output).normalize()
}

/// Run csvmd command with file input
fn run_csvmd_with_file(file_path: &str) -> CliOutput {
    run_csvmd_with_args(&[file_path])
}

/// Run csvmd command with stdin input
fn run_csvmd_with_stdin(input: &str) -> CliOutput {
    let mut child = Command::new("cargo")
        .args(["run", "--"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    CliOutput::from_command_output(output).normalize()
}

#[test]
fn test_cli_with_file_input() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Age,City").unwrap();
    writeln!(temp_file, "John,25,NYC").unwrap();
    writeln!(temp_file, "Jane,30,LA").unwrap();

    let output = run_csvmd_with_file(temp_file.path().to_str().unwrap());
    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_with_stdin() {
    let csv_data = "Product,Price\nLaptop,$999\nMouse,$25";
    let output = run_csvmd_with_stdin(csv_data);
    insta::assert_snapshot!(output.stdout);
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

    let output = run_csvmd_with_file(temp_file.path().to_str().unwrap());
    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_with_empty_file() {
    let temp_file = NamedTempFile::new().unwrap();
    let output = run_csvmd_with_file(temp_file.path().to_str().unwrap());
    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_with_single_column() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Item").unwrap();
    writeln!(temp_file, "Apple").unwrap();
    writeln!(temp_file, "Banana").unwrap();

    let output = run_csvmd_with_file(temp_file.path().to_str().unwrap());
    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_help_flag() {
    let output = run_csvmd_with_args(&["--help"]);
    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_nonexistent_file() {
    let output = run_csvmd_with_args(&["/nonexistent/file.csv"]);
    // Capture full CLI behavior - exit code, stdout, and stderr
    insta::assert_snapshot!(format!(
        "exit_code: {}\nstdout: {}\nstderr: {}",
        output.exit_code, output.stdout, output.stderr
    ));
}

#[test]
fn test_cli_with_unicode() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Symbol,Name,Code").unwrap();
    writeln!(temp_file, "★,Star,U+2605").unwrap();
    writeln!(temp_file, "♠,Spade,U+2660").unwrap();
    writeln!(temp_file, "🚀,Rocket,U+1F680").unwrap();

    let output = run_csvmd_with_file(temp_file.path().to_str().unwrap());
    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_with_mixed_quote_styles() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Quote").unwrap();
    writeln!(temp_file, "Shakespeare,\"To be or not to be\"").unwrap();
    writeln!(temp_file, "Einstein,\"E=mc²\"").unwrap();
    writeln!(temp_file, "Anonymous,No quotes here").unwrap();

    let output = run_csvmd_with_file(temp_file.path().to_str().unwrap());
    insta::assert_snapshot!(output.stdout);
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

#[test]
fn test_cli_with_center_alignment() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Age").unwrap();
    writeln!(temp_file, "John,25").unwrap();
    writeln!(temp_file, "Jane,30").unwrap();

    let output = run_csvmd_with_args(&["--align", "center", temp_file.path().to_str().unwrap()]);

    // Verify successful execution and capture output
    assert_eq!(output.exit_code, 0);
    let expected = "| Name | Age |\n| :---: | :---: |\n| John | 25 |\n| Jane | 30 |\n";
    assert_eq!(output.stdout, expected);
}

#[test]
fn test_cli_with_right_alignment() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Age").unwrap();
    writeln!(temp_file, "John,25").unwrap();

    let output = run_csvmd_with_args(&["--align", "right", temp_file.path().to_str().unwrap()]);

    assert_eq!(output.exit_code, 0);
    let expected = "| Name | Age |\n| ---: | ---: |\n| John | 25 |\n";
    assert_eq!(output.stdout, expected);
}

#[test]
fn test_cli_with_left_alignment() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Age").unwrap();
    writeln!(temp_file, "John,25").unwrap();

    let output = run_csvmd_with_args(&["--align", "left", temp_file.path().to_str().unwrap()]);

    assert_eq!(output.exit_code, 0);
    let expected = "| Name | Age |\n| --- | --- |\n| John | 25 |\n";
    assert_eq!(output.stdout, expected);
}

#[test]
fn test_cli_with_invalid_alignment() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Age").unwrap();
    writeln!(temp_file, "John,25").unwrap();

    let output = run_csvmd_with_args(&["--align", "invalid", temp_file.path().to_str().unwrap()]);

    // Capture complete error behavior including exit code and stderr
    insta::assert_snapshot!(format!(
        "exit_code: {}\nstdout: {}\nstderr: {}",
        output.exit_code, output.stdout, output.stderr
    ));
}

#[test]
fn test_cli_with_streaming_and_alignment() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Age").unwrap();
    writeln!(temp_file, "John,25").unwrap();
    writeln!(temp_file, "Jane,30").unwrap();

    let output = run_csvmd_with_args(&[
        "--stream",
        "--align",
        "center",
        temp_file.path().to_str().unwrap(),
    ]);

    assert_eq!(output.exit_code, 0);
    let expected = "| Name | Age |\n| :---: | :---: |\n| John | 25 |\n| Jane | 30 |\n";
    assert_eq!(output.stdout, expected);
}

#[test]
fn test_cli_with_invalid_utf8_file() {
    let mut temp_file = NamedTempFile::new().unwrap();
    // Write valid CSV header, then invalid UTF-8 bytes
    write!(temp_file, "Name,Age\nJohn,25\n").unwrap();
    temp_file.write_all(&[0x80, 0x81, 0x82]).unwrap();

    let output = run_csvmd_with_file(temp_file.path().to_str().unwrap());

    // Capture complete error behavior for UTF-8 issues
    insta::assert_snapshot!(format!(
        "exit_code: {}\nstdout: {}\nstderr: {}",
        output.exit_code, output.stdout, output.stderr
    ));
}

#[test]
fn test_cli_with_invalid_utf8_streaming_mode() {
    let mut temp_file = NamedTempFile::new().unwrap();
    // Write valid CSV header, then invalid UTF-8 bytes
    write!(temp_file, "Name,Age\nJohn,25\n").unwrap();
    temp_file.write_all(&[0x80, 0x81, 0x82]).unwrap();

    let output = run_csvmd_with_args(&["--stream", temp_file.path().to_str().unwrap()]);

    // Capture streaming mode UTF-8 error behavior
    insta::assert_snapshot!(format!(
        "exit_code: {}\nstdout: {}\nstderr: {}",
        output.exit_code, output.stdout, output.stderr
    ));
}

#[test]
fn test_cli_with_binary_data_in_csv() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Data").unwrap();
    write!(temp_file, "Binary,").unwrap();
    // Write some binary data that will cause UTF-8 parsing issues
    temp_file.write_all(&[0xFF, 0xFE, 0xFD, 0xFC]).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error: Csv"));
    assert!(stderr.contains("invalid utf-8"));
}

#[test]
fn test_cli_with_invalid_utf8_stdin() {
    let mut child = Command::new("cargo")
        .args(["run"])
        .arg("--")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    // Write valid CSV header, then invalid UTF-8 bytes
    let mut stdin_data = Vec::new();
    stdin_data.extend_from_slice(b"Name,Age\nJohn,25\n");
    stdin_data.extend_from_slice(&[0x80, 0x81, 0x82]);

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(&stdin_data)
        .unwrap();

    let result = child.wait_with_output().unwrap();

    assert!(!result.status.success());
    let stderr = String::from_utf8(result.stderr).unwrap();
    assert!(stderr.contains("Error: Csv"));
    assert!(stderr.contains("invalid utf-8"));
}

#[test]
fn test_cli_with_permission_denied_file() {
    // This test only works on Unix-like systems where we can control file permissions
    #[cfg(unix)]
    {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let temp_file = NamedTempFile::new().unwrap();

        // Try to make the file unreadable
        if let Ok(mut perms) = fs::metadata(temp_file.path()).map(|m| m.permissions()) {
            perms.set_mode(0o000);
            if fs::set_permissions(temp_file.path(), perms).is_ok() {
                let output = Command::new("cargo")
                    .args(["run", "--", temp_file.path().to_str().unwrap()])
                    .output()
                    .expect("Failed to execute command");

                assert!(!output.status.success());
                let stderr = String::from_utf8(output.stderr).unwrap();
                assert!(
                    stderr.contains("Permission denied") || stderr.contains("CSV parsing error")
                );
            }
        }
    }

    // On non-Unix systems (like Windows), we'll skip this test since permission
    // manipulation is platform-specific and complex
    #[cfg(not(unix))]
    {
        // Test passes by doing nothing - this ensures cross-platform compatibility
    }
}

#[test]
fn test_cli_with_large_field_causing_memory_error() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Data").unwrap();

    // Create a field that's extremely large to potentially cause issues
    // But not so large that it crashes the test runner
    let large_field = "x".repeat(100_000);
    writeln!(temp_file, "Test,\"{}\"", large_field).unwrap();

    // Add invalid UTF-8 at the end to guarantee an error
    temp_file.write_all(&[0x80, 0x81]).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error: Csv") || stderr.contains("Error: Io"));
}

#[test]
fn test_cli_with_mixed_valid_and_invalid_utf8() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Age,City").unwrap();
    writeln!(temp_file, "John,25,NYC").unwrap();
    writeln!(temp_file, "Jane,30,\"San Francisco\"").unwrap();

    // Add a line with invalid UTF-8 in the middle
    write!(temp_file, "Bob,35,").unwrap();
    temp_file.write_all(&[0xC0, 0xC1]).unwrap(); // Invalid UTF-8 sequence
    writeln!(temp_file).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error: Csv"));
    assert!(stderr.contains("invalid utf-8"));
    // Should provide location information
    assert!(stderr.contains("line") || stderr.contains("record"));
}

// New comprehensive CLI tests for improved coverage

#[test]
fn test_cli_version_flag() {
    let output = run_csvmd_with_args(&["--version"]);
    // Version flag should exit successfully and output version info
    assert_eq!(output.exit_code, 0);
    assert!(output.stdout.contains("csvmd"));
    assert!(output.stderr.is_empty());
}

#[test]
fn test_cli_with_custom_delimiter() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name;Age;City").unwrap();
    writeln!(temp_file, "John;25;NYC").unwrap();
    writeln!(temp_file, "Jane;30;LA").unwrap();

    let output = run_csvmd_with_args(&["--delimiter", ";", temp_file.path().to_str().unwrap()]);

    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_with_no_headers_flag() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "John,25,NYC").unwrap();
    writeln!(temp_file, "Jane,30,LA").unwrap();

    let output = run_csvmd_with_args(&["--no-headers", temp_file.path().to_str().unwrap()]);

    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_with_streaming_flag() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Age").unwrap();
    writeln!(temp_file, "John,25").unwrap();
    writeln!(temp_file, "Jane,30").unwrap();

    let output = run_csvmd_with_args(&["--stream", temp_file.path().to_str().unwrap()]);

    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_combined_flags() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name;Age").unwrap();
    writeln!(temp_file, "John;25").unwrap();
    writeln!(temp_file, "Jane;30").unwrap();

    let output = run_csvmd_with_args(&[
        "--delimiter",
        ";",
        "--stream",
        "--align",
        "right",
        temp_file.path().to_str().unwrap(),
    ]);

    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_stdin_empty_input() {
    let output = run_csvmd_with_stdin("");
    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_stdin_single_line() {
    let output = run_csvmd_with_stdin("Name,Age");
    insta::assert_snapshot!(output.stdout);
}

#[test]
fn test_cli_with_very_large_field() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Name,Description").unwrap();
    let large_content = "x".repeat(10000);
    writeln!(temp_file, "Test,\"{}\"", large_content).unwrap();

    let output = run_csvmd_with_file(temp_file.path().to_str().unwrap());

    // Should handle large fields successfully
    assert_eq!(output.exit_code, 0);
    assert!(output.stdout.contains(&large_content));
    assert!(output.stderr.is_empty());
}

#[test]
fn test_cli_directory_instead_of_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output = run_csvmd_with_args(&[temp_dir.path().to_str().unwrap()]);

    // Should fail when given a directory instead of a file
    insta::assert_snapshot!(format!(
        "exit_code: {}\nstdout: {}\nstderr: {}",
        output.exit_code, output.stdout, output.stderr
    ));
}
