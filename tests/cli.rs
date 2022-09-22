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
        .features("auto-interactive")
        .run()
        .unwrap()
        .command()
}

mod dir {
    use super::*;

    #[test]
    /// `rmd `
    fn missing_operand_error() {
        let mut cmd = no_interactive_bin();
        let assert = cmd.assert();
        assert.stdout(pd::str::contains("missing operand"));
    }

    #[test]
    /// `rmd empty_dir`
    fn remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();

        let assert = cmd.arg(dir.path()).assert();
        assert.stdout(pd::str::contains("Is a directory"));
    }

    #[test]
    /// `rmd -i empty_dir`
    fn interactive_remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();

        let assert = cmd.arg(dir.path()).args(&["-i"]).assert();
        assert.stdout(pd::str::contains("Is a directory"));
    }

    #[test]
    /// `rmd -d empty_dir`
    fn directory_remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();

        let assert = cmd.arg(dir.path()).arg("-d").assert();
        assert.stdout(pd::str::is_empty());
    }

    #[test]
    /// `rmd -d dir`
    fn directory_remove_directory() {
        let dir = TempDir::new().unwrap();
        dir.child("file").touch().unwrap();
        let mut cmd = no_interactive_bin();

        let assert = cmd.arg(dir.path()).arg("-d").assert();
        assert.stdout(pd::str::contains("Directory not empty"));
    }

    #[test]
    /// `rmd -d #empty_dir`
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
    /// `rmd -d #dir`
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
    /// `rmd -id empty_dir`
    fn interactive_directory_remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-d").arg("-i").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("remove directory"));
    }

    #[test]
    /// `rmd -id dir`
    fn interactive_directory_remove_directory() {
        let dir = TempDir::new().unwrap();
        dir.child("file").touch().unwrap();
        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-d").arg("-i").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("Directory not empty"));
    }

    #[test]
    /// `rmd -id #empty_dir`
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
    /// `rmd -id #dir`
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
    /// `rmd -r empty_dir`
    fn recursive_remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-r").arg(dir.path()).assert();
        assert.stdout(pd::str::is_empty());
    }

    #[test]
    /// `rmd -r dir`
    fn recursive_remove_directory() {
        let dir = TempDir::new().unwrap();
        dir.child("file").touch().unwrap();

        let mut cmd = no_interactive_bin();
        let assert = cmd.arg(dir.path()).arg("-r").assert();
        assert.stdout(pd::str::is_empty());
    }

    #[test]
    /// `rmd -r #empty_dir`
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
    /// `rmd -r #dir`
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
    /// `rmd -ri empty_dir`
    fn recursive_interactive_remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-r").arg("-i").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("remove directory"));
    }

    #[test]
    /// `rmd -ri dir`
    fn recursive_interactive_remove_directory() {
        let dir = TempDir::new().unwrap();
        dir.child("file").touch().unwrap();

        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-r").arg("-i").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("descend into directory"));
    }

    #[test]
    /// `rmd -ri #empty_dir`
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
    /// `rmd -ri #dir`
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
    /// `rmd -f empty_dir`
    fn force_remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-f").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("Is a directory"));
    }

    #[test]
    /// `rmd -f dir`
    fn force_remove_directory() {
        let dir = TempDir::new().unwrap();
        dir.child("file").touch().unwrap();

        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-f").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("Is a directory"));
    }

    #[test]
    /// `rmd -f #empty_dir`
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
    /// `rmd -f #dir`
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
    /// `rmd -fd empty_dir`
    fn force_directory_remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-f").arg("-d").arg(dir.path()).assert();
        assert.stdout(pd::str::is_empty());
    }

    #[test]
    /// `rmd -fd dir`
    fn force_directory_remove_directory() {
        let dir = TempDir::new().unwrap();
        dir.child("file").touch().unwrap();
        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-f").arg("-d").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("Directory not empty"));
    }

    #[test]
    /// `rmd -fd #empty_dir`
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
    /// `rmd -fd #dir`
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
    /// `rmd -fdi empty_dir`
    fn force_directory_interactive_remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-f").arg("-d").arg("-i").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("remove directory"));
    }

    #[test]
    /// `rmd -fdi dir`
    fn force_directory_interactive_remove_directory() {
        let dir = TempDir::new().unwrap();
        dir.child("file").touch().unwrap();

        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-f").arg("-d").arg("-i").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("Directory not empty"));
    }

    #[test]
    /// `rmd -fdi #empty_dir`
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
    /// `rmd -fdi #dir`
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
    /// `rmd -rf empty_dir`
    fn force_recursive_remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-f").arg("-r").arg(dir.path()).assert();
        assert.stdout(pd::str::is_empty());
    }

    #[test]
    /// `rmd -rf dir`
    fn force_recursive_remove_directory() {
        let dir = TempDir::new().unwrap();
        dir.child("file").touch().unwrap();

        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-f").arg("-r").arg(dir.path()).assert();
        assert.stdout(pd::str::is_empty());
    }

    #[test]
    /// `rmd -rf #empty_dir`
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
    /// `rmd -rf #dir`
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
    /// `rmd -rfi empty_dir`
    fn force_recursive_interactive_remove_empty_directory() {
        let dir = TempDir::new().unwrap();
        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-f").arg("-r").arg("-i").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("remove directory"));
    }

    #[test]
    /// `rmd -rfi dir`
    fn force_recursive_interactive_remove_directory() {
        let dir = TempDir::new().unwrap();
        dir.child("file").touch().unwrap();

        let mut cmd = no_interactive_bin();
        let assert = cmd.arg("-f").arg("-r").arg("-i").arg(dir.path()).assert();
        assert.stdout(pd::str::contains("descend into directory"));
    }

    #[test]
    /// `rmd -rfi #empty_dir`
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
    /// `rmd -rfi #dir`
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
    /// `rmd -Ir dir dir1`
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
}

mod file {
    use super::*;

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
}

mod flags {
    use super::*;

    #[test]
    fn last_interactivity_flag_wins() {
        let mut cmd = no_interactive_bin();
        let assert = cmd
            .arg("--interactive=never")
            .arg("-i")
            .arg("-I")
            .args(&["file", "file1", "file2", "file3"])
            .assert();
        assert.stdout(pd::str::contains("remove 4 arguments?"));

        let mut cmd = no_interactive_bin();
        let assert = cmd
            .arg("-i")
            .arg("-I")
            .arg("--interactive=always")
            .args(&["file", "file1", "file2", "file3"])
            .assert();
        assert.stdout(pd::str::contains("cannot remove"));
    }
}
