use std::ffi::OsStr;
use std::{fs, path};

use crate::error::Error;
use crate::Result;

pub enum RmStatus<'a> {
    Accept,
    Declined,
    Descend(&'a OsStr),
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

/// # Errors
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
