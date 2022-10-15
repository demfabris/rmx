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
fn shred_removes_empty_file_successfully() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("--shred").arg(&filepath).assert();
    assert.stdout(pd::str::is_empty());
    assert!(!filepath.exists());
}

#[test]
fn shred_removes_file_successfully() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");
    fs::write(&filepath, b"foo bar baz").unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("--shred").arg(&filepath).assert();
    assert.stdout(pd::str::is_empty());
    assert!(!filepath.exists());
}

#[test]
fn shred_doesnt_leave_artifacts() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");
    fs::write(&filepath, b"foo bar baz").unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("--shred").arg(&filepath).assert();
    assert.stdout(pd::str::is_empty());

    let mut leftovers = fs::read_dir(dir).unwrap();

    assert!(leftovers.next().is_none());
}
