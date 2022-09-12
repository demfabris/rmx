#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use dialoguer::{theme, Confirm};
use std::ffi::OsString;
use std::{fs, path};

use clap::ArgMatches;

use arg::{rm_options, InteractiveMode, RmOptions};
use error::Error;

mod arg;
mod error;

pub enum RmStatus {
    Accept,
    Declined,
    Descend(OsString),
    Failed(Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    if let Err(err) = run() {
        match err {
            Error::UnknownEntity(ref file)
            | Error::NoSuchFile(ref file)
            | Error::IsDirectory(ref file)
            | Error::DirectoryNotEmpty(ref file) => {
                println!("rm: cannot remove '{}': {}", file, err);
            }
            _ => (),
        }
    }
}

fn run() -> Result<()> {
    let args = rm_options().get_matches();
    let opt = RmOptions::from(&args);
    let mode = elect_interact_level(&opt, &args);

    if opt == RmOptions::default() && !opt.force {
        println!("rm: missing operand");
        println!("Try 'rm --help' for more information.");
        return Ok(());
    }

    // let mut dir_st = vec![String::new(); 10];
    for path in &opt.file {
        let ent = fs_entity(path)?;
        match ent {
            FsEntity::File { metadata, name } => match prompt_file(&metadata, &name, mode) {
                RmStatus::Accept => {
                    println!("execute");
                }
                RmStatus::Descend(_) | RmStatus::Declined => continue,
                RmStatus::Failed(err) => {
                    return Err(err);
                }
            },

            FsEntity::Dir { metadata, name } => {
                match prompt_dir(&opt, path, &metadata, &name, mode) {
                    RmStatus::Accept => {
                        println!("execute");
                    }
                    RmStatus::Descend(folder) => {
                        println!("descend {:?}", folder);
                    }
                    RmStatus::Declined => continue,
                    RmStatus::Failed(err) => {
                        return Err(err);
                    }
                }
            }

            FsEntity::Symlink { metadata, name } => {
                println!("{:?} {:?}", metadata, name);
                todo!()
            }
        }
    }

    todo!()
}

#[must_use]
/// # Panics
pub fn prompt_dir(
    opt: &RmOptions,
    path: &OsString,
    metadata: &fs::Metadata,
    name: &str,
    mode: InteractiveMode,
) -> RmStatus {
    let is_empty_dir = path::PathBuf::from(path)
        .read_dir()
        .unwrap()
        .next()
        .is_none();

    if !opt.force && !opt.recursive {
        if !opt.dir {
            return RmStatus::Failed(Error::IsDirectory(name.to_owned()));
        }

        if opt.dir && !is_empty_dir {
            return RmStatus::Failed(Error::DirectoryNotEmpty(name.to_owned()));
        }
    }

    let write_protected = is_write_protected(metadata);
    let message = format!(
        "rm: {descend_remove}{write_protected}directory '{name}'?",
        descend_remove = if opt.recursive && !is_empty_dir {
            "descend into"
        } else {
            "remove"
        },
        write_protected = if write_protected {
            " write-protected "
        } else {
            " "
        },
        name = name
    );

    let mut force_accept = false;
    let maybe_interact = match mode {
        InteractiveMode::Always => {
            if is_empty_dir && opt.dir {
                interact_with_message(message)
            } else {
                force_accept = true;
                Ok(false)
            }
        }
        InteractiveMode::Once => {
            force_accept = true;
            Ok(false)
        }
        InteractiveMode::Never => {
            if write_protected {
                interact_with_message(message)
            } else {
                force_accept = true;
                Ok(false)
            }
        }
    };

    if force_accept {
        return RmStatus::Accept;
    }

    if let Ok(yes) = maybe_interact {
        if yes {
            return RmStatus::Accept;
        }
        return RmStatus::Declined;
    }

    RmStatus::Failed(maybe_interact.unwrap_err())
}

#[must_use]
pub fn prompt_file(metadata: &fs::Metadata, name: &str, mode: InteractiveMode) -> RmStatus {
    let write_protected = is_write_protected(metadata);
    let empty = metadata.len() == 0;

    let message = format!(
        "rm: remove{write_protected}regular{empty}file '{name}'?",
        write_protected = if write_protected {
            " write-protected "
        } else {
            " "
        },
        empty = if empty { " empty " } else { " " },
        name = name
    );

    let maybe_interact;
    match mode {
        InteractiveMode::Always => {
            maybe_interact = interact_with_message(message);
        }
        InteractiveMode::Once | InteractiveMode::Never => {
            if write_protected {
                maybe_interact = interact_with_message(message);
            } else {
                return RmStatus::Accept;
            }
        }
    }

    if let Ok(yes) = maybe_interact {
        if yes {
            return RmStatus::Accept;
        }
        return RmStatus::Declined;
    }

    RmStatus::Failed(maybe_interact.unwrap_err())
}

/// # Errors
///
/// Fails with I/O error if can't write to stdout
#[cfg(not(feature = "no-interactive"))]
pub fn interact_with_message(message: String) -> Result<bool> {
    Confirm::with_theme(&theme::ColorfulTheme::default())
        .with_prompt(message)
        .default(true)
        .show_default(true)
        .interact()
        .map_err(std::convert::Into::into)
}

#[cfg(feature = "no-interactive")]
#[allow(clippy::needless_pass_by_value)]
pub fn interact_with_message(message: String) -> Result<bool> {
    println!("{}", message);
    Ok(true)
}

#[cfg(unix)]
#[must_use]
pub fn is_write_protected(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;

    let file_uid = metadata.uid();
    let proc_uid = unsafe { libc::getuid() };

    metadata.permissions().readonly() || file_uid != proc_uid
}

#[cfg(windows)]
#[must_use]
pub fn is_write_protected(metadata: &fs::Metadata) -> bool {
    metadata.permissions().readonly()
}

/// Get the last occurence of a flag and return its index
#[inline]
fn last_flag_occurence(indices_of: Option<clap::Indices>, is_present: bool) -> usize {
    if is_present {
        *indices_of
            .map(std::iter::Iterator::collect::<Vec<usize>>)
            .unwrap_or_default()
            .last()
            .unwrap_or(&0)
    } else {
        0
    }
}

#[must_use]
#[inline]
pub fn elect_interact_level(opt: &RmOptions, args: &ArgMatches) -> InteractiveMode {
    let flag_always = last_flag_occurence(
        args.indices_of("interactive_always"),
        opt.interactive_always,
    );
    let flag_once = last_flag_occurence(args.indices_of("interactive_once"), opt.interactive_once);
    let flag_mode = last_flag_occurence(args.indices_of("WHEN"), true);

    if flag_always > flag_once && flag_always > flag_mode {
        InteractiveMode::Always
    } else if flag_once > flag_always && flag_once > flag_mode {
        InteractiveMode::Once
    } else if flag_mode > flag_always && flag_mode > flag_once {
        opt.interactive
    } else {
        InteractiveMode::Never
    }
}

#[derive(Debug)]
pub enum FsEntity {
    Symlink {
        metadata: fs::Metadata,
        name: String,
    },
    Dir {
        metadata: fs::Metadata,
        name: String,
    },
    File {
        metadata: fs::Metadata,
        name: String,
    },
}

/// # Errors
pub fn fs_entity(path: &OsString) -> Result<FsEntity> {
    let name = path::PathBuf::from(path)
        .file_name()
        .map(|t| t.to_string_lossy().into_owned())
        .unwrap_or_default();
    let metadata = fs::metadata(path).map_err(|_| Error::NoSuchFile(name.clone()))?;

    let entity = match metadata {
        m if m.is_dir() => FsEntity::Dir { metadata: m, name },
        m if m.is_file() => FsEntity::File { metadata: m, name },
        m if m.is_symlink() => FsEntity::Symlink { metadata: m, name },
        _ => {
            return Err(Error::UnknownEntity(name));
        }
    };

    Ok(entity)
}

#[cfg(test)]
/// Tests naming structure
/// test cli + flags + operation + adjs.
mod tests {
    use assert_cmd::prelude::*;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use escargot::CargoBuild;
    use predicates as pd;

    use super::*;

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
}
