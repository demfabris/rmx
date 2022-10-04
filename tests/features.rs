use std::fs;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use escargot::CargoBuild;

/// Build `rmx` bin that accepts every command line interaction
fn no_interactive_bin() -> std::process::Command {
    CargoBuild::new()
        .bin("rmx")
        .features("auto-interactive")
        .run()
        .unwrap()
        .command()
}

use std::ffi::OsString;

#[test]
fn flatten_level_1() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    dir.child("dir1").create_dir_all().unwrap();
    dir.child("dir1/file2").touch().unwrap();

    let mut cmd = no_interactive_bin();
    cmd.arg(dir.path())
        .arg("--flatten")
        .arg("1")
        .output()
        .unwrap();

    let mut entries = dir.read_dir().unwrap();

    assert_eq!(
        entries.next().unwrap().unwrap().file_name(),
        OsString::from("file2")
    );
    assert_eq!(
        entries.next().unwrap().unwrap().file_name(),
        OsString::from("file")
    );
}

#[test]
fn flatten_level_1_name_conflict() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    dir.child("file1").touch().unwrap();
    dir.child("dir1").create_dir_all().unwrap();
    dir.child("dir1/file2").touch().unwrap();
    dir.child("dir1/file1").touch().unwrap();

    let mut cmd = no_interactive_bin();
    cmd.arg(dir.path())
        .arg("--flatten")
        .arg("1")
        .output()
        .unwrap();

    let mut entries = dir.read_dir().unwrap();

    assert_eq!(
        entries.next().unwrap().unwrap().file_name(),
        OsString::from("file2")
    );
    assert_eq!(
        entries.next().unwrap().unwrap().file_name(),
        OsString::from("file")
    );
    // `dir1` won't be removed because `dir/file1` exists
    assert_eq!(
        entries.next().unwrap().unwrap().file_name(),
        OsString::from("dir1")
    );
    assert_eq!(
        entries.next().unwrap().unwrap().file_name(),
        OsString::from("file1")
    );

    // `dir1/file1` is skipped
    let mut dir1_entries = fs::read_dir(dir.path().join("dir1")).unwrap();
    let file1 = dir1_entries.next().unwrap().unwrap().file_name();
    assert_eq!(file1, "file1");
}

#[test]
fn flatten_level_0() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    dir.child("file1").touch().unwrap();
    dir.child("dir1").create_dir_all().unwrap();
    dir.child("dir1/file2").touch().unwrap();

    let mut cmd = no_interactive_bin();
    cmd.arg(dir.path())
        .arg("--flatten")
        .arg("0")
        .output()
        .unwrap();

    let mut entries = dir.read_dir().unwrap();

    assert_eq!(
        entries.next().unwrap().unwrap().file_name(),
        OsString::from("file2")
    );
    assert_eq!(
        entries.next().unwrap().unwrap().file_name(),
        OsString::from("file")
    );
    assert_eq!(
        entries.next().unwrap().unwrap().file_name(),
        OsString::from("file1")
    );
}
