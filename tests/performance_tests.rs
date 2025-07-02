use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_streaming_performance_with_large_data() {
    // Generate a medium-sized CSV for testing
    let mut temp_file = NamedTempFile::new().unwrap();

    // Write headers
    writeln!(temp_file, "ID,Name,Description,Value").unwrap();

    // Write 1000 rows of data
    for i in 0..1000 {
        writeln!(
            temp_file,
            "{},Name_{},\"Description with | pipes and\nnewlines\",{}",
            i,
            i,
            i * 10
        )
        .unwrap();
    }

    let start = std::time::Instant::now();
    let output = Command::new("cargo")
        .args(["run", "--", "--stream", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let duration = start.elapsed();

    assert!(output.status.success());
    assert!(
        duration.as_millis() < 2000,
        "Streaming should complete within 2 seconds for 1000 rows"
    );

    // Verify output format
    let result = String::from_utf8(output.stdout).unwrap();
    assert!(result.starts_with("| ID | Name | Description | Value |"));
    assert!(result.contains("| --- | --- | --- | --- |"));
    assert!(result.contains("Name_999"));
}

#[test]
fn test_streaming_with_variable_columns() {
    let mut temp_file = NamedTempFile::new().unwrap();

    // CSV with variable column counts
    writeln!(temp_file, "A,B").unwrap();
    writeln!(temp_file, "1,2,3").unwrap();
    writeln!(temp_file, "4,5,6,7").unwrap();
    writeln!(temp_file, "8,9").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", "--stream", temp_file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let result = String::from_utf8(output.stdout).unwrap();
    // Should have 4 columns (max from any row)
    assert!(result.contains("| --- | --- | --- | --- |"));
    // Should handle uneven rows correctly
    assert!(result.contains("| 8 | 9 |  |  |"));
}
