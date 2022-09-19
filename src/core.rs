use std::ffi::OsStr;
use std::{fs, io, path};

use crate::arg::RmOptions;
use crate::error::Error;
use crate::Result;

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

pub fn fs_entity(path: &OsStr) -> Result<FsEntity> {
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
