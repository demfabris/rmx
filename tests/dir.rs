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
/// `rmx `
fn missing_operand_error() {
    let mut cmd = no_interactive_bin();
    let assert = cmd.assert();
    assert.stdout(pd::str::contains("missing operand"));
}

#[test]
/// `rmx empty_dir`
fn remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmx -i empty_dir`
fn interactive_remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir.path()).args(&["-i"]).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmx -d empty_dir`
fn directory_remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir.path()).arg("-d").assert();
    assert.stdout(pd::str::is_empty());
}

#[test]
/// `rmx -d dir`
fn directory_remove_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir.path()).arg("-d").assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmx -d #empty_dir`
fn directory_remove_write_protected_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(dir.path()).arg("-d").assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmx -d #dir`
fn directory_remove_write_protected_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir.path()).arg("-d").assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmx -id empty_dir`
fn interactive_directory_remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-d").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("remove directory"));
}

#[test]
/// `rmx -id dir`
fn interactive_directory_remove_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-d").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmx -id #empty_dir`
fn interactive_remove_write_protected_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-d").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmx -id #dir`
fn interactive_remove_write_protected_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-d").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmx -r empty_dir`
fn recursive_remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-r").arg(dir.path()).assert();
    assert.stdout(pd::str::is_empty());
}

#[test]
/// `rmx -r dir`
fn recursive_remove_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(dir.path()).arg("-r").assert();
    assert.stdout(pd::str::is_empty());
}

#[test]
/// `rmx -r #empty_dir`
fn recursive_remove_write_protected_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(dir.path()).arg("-r").assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmx -r #dir`
fn recursive_remove_write_protected_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(dir.path()).arg("-r").assert();
    assert.stdout(pd::str::contains("descend into write-protected directory"));
}

#[test]
/// `rmx -ri empty_dir`
fn recursive_interactive_remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-r").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("remove directory"));
}

#[test]
/// `rmx -ri dir`
fn recursive_interactive_remove_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-r").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("descend into directory"));
}

#[test]
/// `rmx -ri #empty_dir`
fn recursive_interactive_remove_write_protected_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-r").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmx -ri #dir`
fn recursive_interactive_remove_write_protected_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-r").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("descend into write-protected directory"));
}

#[test]
/// `rmx -f empty_dir`
fn force_remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmx -f dir`
fn force_remove_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmx -f #empty_dir`
fn force_remove_write_protected_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmx -f #dir`
fn force_remove_write_protected_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmx -fd empty_dir`
fn force_directory_remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg(dir.path()).assert();
    assert.stdout(pd::str::is_empty());
}

#[test]
/// `rmx -fd dir`
fn force_directory_remove_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmx -fd #empty_dir`
fn force_directory_remove_write_protected_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Operation not permitted"));
}

#[test]
/// `rmx -fd #dir`
fn force_directory_remove_write_protected_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmx -fdi empty_dir`
fn force_directory_interactive_remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("remove directory"));
}

#[test]
/// `rmx -fdi dir`
fn force_directory_interactive_remove_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmx -fdi #empty_dir`
fn force_directory_interactive_remove_write_protected_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmx -fdi #dir`
fn force_directory_interactive_remove_write_protected_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmx -rf empty_dir`
fn force_recursive_remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg(dir.path()).assert();
    assert.stdout(pd::str::is_empty());
}

#[test]
/// `rmx -rf dir`
fn force_recursive_remove_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg(dir.path()).assert();
    assert.stdout(pd::str::is_empty());
}

#[test]
/// `rmx -rf #empty_dir`
fn force_recursive_remove_write_protected_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Operation not permitted"));
}

#[test]
/// `rmx -rf #dir`
fn force_recursive_remove_write_protected_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("Permission denied"));
}

#[test]
/// `rmx -rfi empty_dir`
fn force_recursive_interactive_remove_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("remove directory"));
}

#[test]
/// `rmx -rfi dir`
fn force_recursive_interactive_remove_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("descend into directory"));
}

#[test]
/// `rmx -rfi #empty_dir`
fn force_recursive_interactive_remove_write_protected_empty_directory() {
    let dir = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmx -rfi #dir`
fn force_recursive_interactive_remove_write_protected_directory() {
    let dir = TempDir::new().unwrap();
    dir.child("file").touch().unwrap();
    let mut perms = fs::metadata(dir.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg("-i").arg(dir.path()).assert();
    assert.stdout(pd::str::contains("descend into write-protected directory"));
}

#[test]
/// `rmx -Ir dir dir1`
fn interactive_once_remove_two_empty_directories_recursively() {
    let dir = TempDir::new().unwrap();
    let dir1 = TempDir::new().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd
        .arg("-I")
        .arg("-r")
        .args(&[dir.path(), dir1.path()])
        .assert();
    assert.stdout(pd::str::contains("remove 2 arguments recursively?"));
}

#[test]
fn preserve_root_equals_dir_remove_directory() {
    let dir = TempDir::new().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd
        .arg(format!("--preserve-root={}", dir.path().display()))
        .arg("-d")
        .args(&[dir.path(), dir.path()])
        .assert();
    assert.stdout(pd::str::contains("refusing to remove"));
}
