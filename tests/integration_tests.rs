use assert_cmd::Command;
use std::fs;

#[test]
fn test_anew_stdout() {
    let mut cmd = Command::cargo_bin("anew").unwrap();
    cmd.args(&["-q", "-t"])
        .write_stdin("line1\nline2\nline1\nline3\n");
    cmd.assert().success().stdout(""); // Quiet mode; no output
}

#[test]
fn test_anew_file_append() {
    let test_file = "test_output.txt";
    let _ = fs::remove_file(test_file);

    // First run to create the file
    let mut cmd = Command::cargo_bin("anew").unwrap();
    cmd.args(&["-t"])
        .arg(test_file)
        .write_stdin(" line1 \nline2\nline1\nline3\n")
        .assert()
        .success();

    // Second run to append new lines
    let mut cmd = Command::cargo_bin("anew").unwrap();
    cmd.args(&["-q"])
        .arg(test_file)
        .write_stdin("line2\nline4\n")
        .assert()
        .success();

    let content = fs::read_to_string(test_file).unwrap();
    assert!(content.contains("line1\n"));
    assert!(content.contains("line2\n"));
    assert!(content.contains("line3\n"));
    assert!(content.contains("line4\n"));

    let _ = fs::remove_file(test_file);
}

#[test]
fn test_trim_option() {
    let mut cmd = Command::cargo_bin("anew").unwrap();
    cmd.args(&["-t"])
        .write_stdin("  line1  \nline1\nline2\n")
        .assert()
        .success()
        .stdout("line1\nline2\n");
}

#[test]
fn test_dry_run() {
    let test_file = "test_dry_run_output.txt";
    let _ = fs::remove_file(test_file);

    let mut cmd = Command::cargo_bin("anew").unwrap();
    cmd.args(&["-d", "-q", "-t", test_file])
        .write_stdin("line1\nline2\n")
        .assert()
        .success();

    assert!(!fs::metadata(test_file).is_ok());
}
