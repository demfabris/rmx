use std::ffi::{OsStr, OsString};
use std::{fs, io, path};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use crate::arg::RmOptions;
use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub const BIN_NAME: &str = env!("CARGO_BIN_NAME");

pub enum RmStatus {
    Accept,
    Declined,
    Failed(Error),
}

#[derive(Debug)]
pub enum FsEntity {
    Symlink {
        metadata: fs::Metadata,
        name: String,
        inode_id: u64,
    },
    Dir {
        metadata: fs::Metadata,
        name: String,
        inode_id: u64,
    },
    File {
        metadata: fs::Metadata,
        name: String,
        inode_id: u64,
    },
}

#[cfg(unix)]
#[must_use]
pub fn is_write_protected(metadata: &fs::Metadata) -> bool {
    let file_uid = metadata.uid();
    let proc_uid = unsafe { libc::getuid() };

    metadata.permissions().readonly() || file_uid != proc_uid
}

#[cfg(windows)]
#[must_use]
pub fn is_write_protected(metadata: &fs::Metadata) -> bool {
    metadata.permissions().readonly()
}

pub fn is_empty_dir(path: &OsStr) -> bool {
    fs::read_dir(path)
        .expect("path to be a directory")
        .next()
        .is_none()
}

pub fn concat_relative_root(rel_root: &str, name: &str) -> String {
    format!(
        "{}{}{}",
        &rel_root,
        if rel_root.is_empty() { "" } else { "/" },
        &name
    )
}

pub fn unlink_dir(
    path: &OsStr,
    name: &str,
    rel_root: &str,
    visited: bool,
    opt: &RmOptions,
) -> Result<bool> {
    if !is_empty_dir(path) && !visited {
        return Ok(false);
    }

    if is_write_protected(&fs::metadata(path)?) {
        let relative_name = concat_relative_root(rel_root, name);
        return Err(Error::OperationNotPermitted(relative_name));
    }

    fs::remove_dir(path).map_err(|err| match err.kind() {
        io::ErrorKind::PermissionDenied => {
            let relative_name = concat_relative_root(rel_root, name);
            Error::OperationNotPermitted(relative_name)
        }
        _ => Error::Io(err),
    })?;

    if opt.verbose {
        let relative_name = concat_relative_root(rel_root, name);
        println!("directory '{}' was removed", relative_name);
    }

    Ok(true)
}

pub fn unlink_file(path: &OsStr, name: &str, rel_root: &str, opt: &RmOptions) -> Result<()> {
    fs::remove_file(path).map_err(|err| match err.kind() {
        io::ErrorKind::PermissionDenied => {
            let relative_name = concat_relative_root(rel_root, name);
            Error::PermissionDenied(relative_name)
        }
        _ => Error::Io(err),
    })?;

    if opt.verbose {
        let relative_name = concat_relative_root(rel_root, name);
        println!("removed '{}'", relative_name);
    }

    Ok(())
}

pub fn unlink_symlink(path: &OsStr, name: &str, rel_root: &str, opt: &RmOptions) -> Result<()> {
    fs::remove_file(path)?;
    let relative_name = concat_relative_root(rel_root, name);
    if opt.verbose {
        println!("removed '{}'", relative_name);
    }

    Ok(())
}

pub fn fs_entity(path: &OsStr) -> Result<FsEntity> {
    let name = path::Path::new(path)
        .file_name()
        .map(|t| t.to_string_lossy().into_owned())
        .unwrap_or_default();
    let metadata = fs::symlink_metadata(path).map_err(|_| Error::NoSuchFile(name.clone()))?;

    #[cfg(unix)]
    let inode_id = metadata.dev();

    #[cfg(not(unix))]
    let inode_id = 0_u64;

    let entity = match metadata {
        m if m.is_dir() => FsEntity::Dir {
            metadata: m,
            name,
            inode_id,
        },
        m if m.is_symlink() => FsEntity::Symlink {
            metadata: m,
            name,
            inode_id,
        },
        m if m.is_file() => FsEntity::File {
            metadata: m,
            name,
            inode_id,
        },
        _ => {
            return Err(Error::UnknownEntity(name));
        }
    };

    Ok(entity)
}

pub fn one_file_system(opt: &RmOptions, fullname: &str, parent: u64, child: u64) -> bool {
    // This is either top path or we're not on unix
    if parent == 0 {
        return false;
    }

    #[cfg(unix)]
    if opt.one_file_system && parent != child {
        println!(
            "rm: skipping '{fullname}', since it's on a different device",
            fullname = fullname
        );
        true
    } else {
        false
    }

    #[cfg(not(unix))]
    false
}

pub fn preserve_root(opt: &RmOptions, path: &OsStr) -> bool {
    #[cfg(not(any(windows, unix)))]
    {
        println!("rm: unsupported");
        return true;
    }

    if opt.no_preserve_root {
        return false;
    }

    #[cfg(unix)]
    let pred = if opt.preserve_root == "all" {
        OsString::from("/")
    } else {
        opt.preserve_root.clone()
    };

    #[cfg(windows)]
    let pred = if opt.preserve_root == "all" {
        OsString::from("C:\\")
    } else {
        opt.preserve_root.clone()
    };

    let maybe_path = path::Path::new(path).canonicalize().ok();
    let maybe_pred = path::Path::new(&pred).canonicalize().ok();

    if let (Some(fullpath), Some(fullpred)) = (maybe_path, maybe_pred) {
        if fullpath == fullpred {
            println!(
                "rm: refusing to remove '{path}': skipping (preserve-root='{pred}')",
                path = path.to_string_lossy(),
                pred = pred.to_string_lossy()
            );
            return true;
        }
    }

    false
}
