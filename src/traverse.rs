use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::ffi::CString;

use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};

use crate::arg::{InteractiveMode, RmOptions};
use crate::core::{
    concat_relative_root, fs_entity, one_file_system, preserve_root, unlink_dir, unlink_file,
    unlink_symlink, FsEntity, Result, RmStatus,
};
use crate::{dir, file, link};

pub fn dfs(
    path: &OsStr,
    rel_root: String,
    opt: &RmOptions,
    mode: InteractiveMode,
    visited: bool,
    parent_inode_id: u64,
) -> Result<()> {
    let ent = fs_entity(path);

    if let Err(err) = ent {
        println!("{}", err);
        return Ok(());
    }

    match ent? {
        FsEntity::File {
            metadata,
            name,
            inode_id,
        } => match file::prompt(&metadata, &name, &rel_root, mode) {
            RmStatus::Accept => {
                if one_file_system(opt, &name, parent_inode_id, inode_id) {
                    return Ok(());
                }

                unlink_file(path, &name, &rel_root, opt)?;
            }
            RmStatus::Declined => return Ok(()),
            RmStatus::Failed(err) => return Err(err),
        },

        FsEntity::Dir {
            metadata,
            name,
            inode_id,
        } => {
            match dir::prompt(opt, path, &rel_root, &metadata, &name, mode, visited) {
                RmStatus::Accept => {
                    if preserve_root(opt, path) {
                        return Ok(());
                    }

                    if !unlink_dir(path, &name, &rel_root, visited, opt)? {
                        for entry in fs::read_dir(path)? {
                            let path = entry?.path();
                            let rel_root = concat_relative_root(&rel_root, &name);

                            if one_file_system(opt, &rel_root, parent_inode_id, inode_id) {
                                return Ok(());
                            }
                            dfs(path.as_os_str(), rel_root, opt, mode, false, inode_id)?;
                        }
                        // Parent folder is deleted last
                        dfs(path, rel_root, opt, mode, true, inode_id)?;
                    }
                }
                RmStatus::Declined => return Ok(()),
                RmStatus::Failed(err) => return Err(err),
            }
        }

        FsEntity::Symlink { name, inode_id, .. } => match link::prompt(&name, &rel_root, mode) {
            RmStatus::Accept => {
                if one_file_system(opt, &name, parent_inode_id, inode_id) {
                    return Ok(());
                }

                if opt.follow_symlinks {
                    let resolved_path = fs::read_link(path)?;
                    dfs(
                        resolved_path.as_os_str(),
                        String::new(),
                        opt,
                        mode,
                        true,
                        parent_inode_id,
                    )?;
                }

                unlink_symlink(path, &name, &rel_root, opt)?;
            }
            RmStatus::Declined => return Ok(()),
            RmStatus::Failed(err) => return Err(err),
        },
    }

    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::while_let_loop)]
pub fn walk(path: &OsStr) -> Result<()> {
    let mut dirs: BTreeMap<usize, Vec<PathBuf>> = BTreeMap::new();
    let (tx, rx): (Sender<PathBuf>, Receiver<PathBuf>) = unbounded();

    let handle = std::thread::spawn(move || loop {
        match rx.recv() {
            Ok(path) => {
                #[cfg(unix)]
                {
                    let c_path = unsafe {
                        CString::new(path.to_str().unwrap_unchecked()).unwrap_unchecked()
                    };
                    unsafe { if libc::unlink(c_path.as_ptr()) == -1 {} };
                }

                #[cfg(windows)]
                fs::remove_file(path).unwrap();
            }
            _ => break,
        }
    });

    jwalk::WalkDir::new(path)
        .skip_hidden(false)
        .into_iter()
        .for_each(|t| {
            let t = unsafe { t.unwrap_unchecked() };
            let (path, depth) = (t.path(), t.depth);

            if path.is_dir() {
                dirs.entry(depth).or_insert_with(Vec::new).push(path);
            } else {
                unsafe { tx.send(path).unwrap_unchecked() };
            }
        });

    drop(tx);
    unsafe { handle.join().unwrap_unchecked() };

    for (_, dir) in dirs.iter().rev() {
        #[cfg(unix)]
        for d in dir {
            let c_path = unsafe { CString::new(d.to_str().unwrap_unchecked()).unwrap_unchecked() };
            unsafe { if libc::rmdir(c_path.as_ptr()) == -1 {} };
        }

        #[cfg(windows)]
        for _ in dir {
            fs::remove_dir(path).unwrap();
        }
    }

    Ok(())
}
