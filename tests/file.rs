use std::fs;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use escargot::CargoBuild;
use predicates as pd;

/// Build `rmx` bin that accepts every command line interaction
fn no_interactive_bin() -> std::process::Command {
    CargoBuild::new()
        .bin("rmx")
        .features("auto-interactive")
        .run()
        .unwrap()
        .command()
}

#[test]
/// `rm empty_file`
fn remove_empty_file() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(&filepath).assert();
    assert.stdout(pd::str::is_empty());
}

#[test]
/// `rm file`
fn remove_file() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");
    fs::write(&filepath, b"Matthew McConaughey").unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(&filepath).assert();
    assert.stdout(pd::str::is_empty());
}

#[test]
/// `rm #empty_file`
fn remove_write_protected_empty_file() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");
    let mut perms = fs::metadata(&filepath).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&filepath, perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(&filepath).assert();
    assert.stdout(pd::str::contains(
        "remove write-protected regular empty file",
    ));
}

#[test]
/// `rm #file`
fn remove_write_protected_file() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");
    fs::write(&filepath, b"Matthew McConaughey").unwrap();
    let mut perms = fs::metadata(&filepath).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&filepath, perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(&filepath).assert();
    assert.stdout(pd::str::contains("remove write-protected regular file"));
}

#[test]
/// `rm -i empty_file`
fn interactive_remove_empty_file() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(&filepath).arg("-i").assert();
    assert.stdout(pd::str::contains("remove regular empty file"));
}

#[test]
/// `rm -i file`
fn interactive_remove_file() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");
    fs::write(&filepath, b"Matthew McConaughey").unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(&filepath).arg("-i").assert();
    assert.stdout(pd::str::contains("remove regular file"));
}

#[test]
/// `rm -i #empty_file`
fn interactive_remove_write_protected_empty_file() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");
    let mut perms = fs::metadata(&filepath).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&filepath, perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(&filepath).arg("-i").assert();
    assert.stdout(pd::str::contains(
        "remove write-protected regular empty file",
    ));
}

#[test]
/// `rm -i #file`
fn interactive_remove_write_protected_file() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");
    fs::write(&filepath, b"Matthew McConaughey").unwrap();
    let mut perms = fs::metadata(&filepath).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&filepath, perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(&filepath).arg("-i").assert();
    assert.stdout(pd::str::contains("remove write-protected regular file"));
}

#[test]
/// `rm -I file file1 file2 file3`
fn interactive_once_remove_four_empty_files() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    dir.child("file1").touch().unwrap();
    dir.child("file2").touch().unwrap();
    dir.child("file3").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd
        .arg("-I")
        .args(&["file", "file1", "file2", "file3"])
        .assert();
    assert.stdout(pd::str::contains("remove 4 arguments?"));
}
