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

#[cfg(unix)]
use std::os::unix::fs::symlink;

#[test]
#[cfg(unix)]
fn interactive_remove_symlink() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");
    let link = dir.path().join("link");
    symlink(&filepath, &link).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-i").arg(&link).assert();
    assert.stdout(pd::str::contains("remove symbolic link"));
}

#[test]
#[cfg(unix)]
fn interactive_follow_symlink_remove_symlink() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let filepath = dir.path().join("file");
    let link = dir.path().join("link");
    symlink(&filepath, &link).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-i").arg("-l").arg(&link).assert();
    assert.stdout(pd::str::contains("remove regular empty file"));
}
