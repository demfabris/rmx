/// Tests naming structure
/// test cli + flags + operation + adjs.
use std::fs;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use escargot::CargoBuild;
use predicates as pd;

/// Build `rmd` bin that accepts every command line interaction
fn no_interactive_bin() -> std::process::Command {
    CargoBuild::new()
        .bin("rmd")
        .features("no-interactive")
        .run()
        .unwrap()
        .command()
}

#[test]
/// `rmd `
fn test_cli_missing_operand_error() {
    let mut cmd = no_interactive_bin();
    let assert = cmd.assert();
    assert.stdout(pd::str::contains("missing operand"));
}

#[test]
/// `rmd empty_dir`
fn test_cli_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmd -i empty_dir`
fn test_cli_interactive_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir1.path()).args(&["-i"]).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmd -d empty_dir`
fn test_cli_directory_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir1.path()).arg("-d").assert();
    assert.stdout(pd::str::contains("execute"));
}

#[test]
/// `rmd -d dir`
fn test_cli_directory_remove_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir1.path()).arg("-d").assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmd -d #empty_dir`
fn test_cli_directory_remove_write_protected_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir1.path()).arg("-d").assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmd -d #dir`
fn test_cli_directory_remove_write_protected_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();
    let mut cmd = no_interactive_bin();

    let assert = cmd.arg(dir1.path()).arg("-d").assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmd -id empty_dir`
fn test_cli_interactive_directory_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-d").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("remove directory"));
}

#[test]
/// `rmd -id dir`
fn test_cli_interactive_directory_remove_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-d").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmd -id #empty_dir`
fn test_cli_interactive_remove_write_protected_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-d").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmd -id #dir`
fn test_cli_interactive_remove_write_protected_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-d").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmd -r empty_dir`
fn test_cli_recursive_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-r").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("execute"));
}

#[test]
/// `rmd -r dir`
fn test_cli_recursive_remove_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(dir1.path()).arg("-r").assert();
    assert.stdout(pd::str::contains("execute"));
}

#[test]
/// `rmd -r #empty_dir`
fn test_cli_recursive_remove_write_protected_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(dir1.path()).arg("-r").assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmd -r #dir`
fn test_cli_recursive_remove_write_protected_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg(dir1.path()).arg("-r").assert();
    assert.stdout(pd::str::contains("descend into write-protected directory"));
}

#[test]
/// `rmd -ri empty_dir`
fn test_cli_recursive_interactive_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-r").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("remove directory"));
}

#[test]
/// `rmd -ri dir`
fn test_cli_recursive_interactive_remove_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-r").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("descend into directory"));
}

#[test]
/// `rmd -ri #empty_dir`
fn test_cli_recursive_interactive_remove_write_protected_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-r").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmd -ri #dir`
fn test_cli_recursive_interactive_remove_write_protected_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-r").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("descend into write-protected directory"));
}

#[test]
/// `rmd -f empty_dir`
fn test_cli_force_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmd -f dir`
fn test_cli_force_remove_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmd -f #empty_dir`
fn test_cli_force_remove_write_protected_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmd -f #dir`
fn test_cli_force_remove_write_protected_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Is a directory"));
}

#[test]
/// `rmd -fd empty_dir`
fn test_cli_force_directory_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("execute"));
}

#[test]
/// `rmd -fd dir`
fn test_cli_force_directory_remove_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmd -fd #empty_dir`
fn test_cli_force_directory_remove_write_protected_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("execute")); // Operation not permitted
}

#[test]
/// `rmd -fd #dir`
fn test_cli_force_directory_remove_write_protected_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmd -fdi empty_dir`
fn test_cli_force_directory_interactive_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("remove directory"));
}

#[test]
/// `rmd -fdi dir`
fn test_cli_force_directory_interactive_remove_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmd -fdi #empty_dir`
fn test_cli_force_directory_interactive_remove_write_protected_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmd -fdi #dir`
fn test_cli_force_directory_interactive_remove_write_protected_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-d").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("Directory not empty"));
}

#[test]
/// `rmd -rf empty_dir`
fn test_cli_force_recursive_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("execute"));
}

#[test]
/// `rmd -rf dir`
fn test_cli_force_recursive_remove_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("execute"));
}

#[test]
/// `rmd -rf #empty_dir`
fn test_cli_force_recursive_remove_write_protected_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("execute"));
}

#[test]
/// `rmd -rf #dir`
fn test_cli_force_recursive_remove_write_protected_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("execute"));
}

#[test]
/// `rmd -rfi empty_dir`
fn test_cli_force_recursive_interactive_remove_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("remove directory"));
}

#[test]
/// `rmd -rfi dir`
fn test_cli_force_recursive_interactive_remove_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("descend into directory"));
}

#[test]
/// `rmd -rfi #empty_dir`
fn test_cli_force_recursive_interactive_remove_write_protected_empty_directory() {
    let dir1 = TempDir::new().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("remove write-protected directory"));
}

#[test]
/// `rmd -rfi #dir`
fn test_cli_force_recursive_interactive_remove_write_protected_directory() {
    let dir1 = TempDir::new().unwrap();
    dir1.child("file1").touch().unwrap();
    let mut perms = fs::metadata(dir1.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir1.path(), perms).unwrap();

    let mut cmd = no_interactive_bin();
    let assert = cmd.arg("-f").arg("-r").arg("-i").arg(dir1.path()).assert();
    assert.stdout(pd::str::contains("descend into write-protected directory"));
}
