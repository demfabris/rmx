use std::collections::BTreeMap;
use std::ffi::CString;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};

use crate::arg::{InteractiveMode, RmOptions};
use crate::core::{
    concat_relative_root, fs_entity, one_file_system, preserve_root, unlink_dir, unlink_file,
    FsEntity, Result, RmStatus,
};
use crate::{dir, file};

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

        FsEntity::Symlink {
            metadata,
            name,
            inode_id,
        } => {
            println!("{:?} {:?} {:?}", metadata, name, inode_id);
            todo!()
        }
    }

    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::while_let_loop)]
pub fn walk(_opt: &RmOptions, path: &OsStr) -> Result<()> {
    let mut dirs: BTreeMap<usize, Vec<PathBuf>> = BTreeMap::new();
    let (tx, rx): (Sender<PathBuf>, Receiver<PathBuf>) = channel();

    let handle = std::thread::spawn(move || loop {
        match rx.recv() {
            Ok(path) => {
                #[cfg(unix)]
                {
                    let c_path = CString::new(path.to_str().unwrap()).unwrap();
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
                tx.send(path).unwrap();
            }
        });

    drop(tx);
    handle.join().unwrap();

    for (_, dir) in dirs.iter().rev() {
        for d in dir {
            #[cfg(unix)]
            {
                let c_path = CString::new(d.to_str().unwrap()).unwrap();
                unsafe { if libc::rmdir(c_path.as_ptr()) == -1 {} };
            }

            #[cfg(windows)]
            fs::remove_dir(path).unwrap();
        }
    }

    Ok(())
}
