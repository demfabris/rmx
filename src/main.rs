#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::ffi::OsStr;
use std::fs;

use crate::arg::{elect_interact_level, rm_options, InteractiveMode, RmOptions};
use crate::core::{
    concat_relative_root, fs_entity, is_different_fs, unlink_dir, unlink_file, FsEntity, RmStatus,
};
use error::Error;

mod arg;
mod core;
mod dir;
mod error;
mod file;
mod interact;

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}

fn run() -> Result<()> {
    let args = rm_options().get_matches();
    let opt = RmOptions::from(&args);
    let mode = elect_interact_level(&opt, &args);

    if opt == RmOptions::default() && !opt.force {
        return Err(Error::Usage);
    }

    if mode == InteractiveMode::Once && (opt.file.len() > 3 || opt.recursive) {
        let message = format!(
            "rm: remove {count} {arguments}{recursive}?",
            count = opt.file.len(),
            arguments = if opt.file.len() == 1 {
                "argument"
            } else {
                "arguments"
            },
            recursive = if opt.recursive { " recursively" } else { "" }
        );

        match interact::with_message(message) {
            Ok(true) => (),
            Err(err) => return Err(err),
            _ => return Ok(()),
        }
    }

    for path in &opt.file {
        traverse(path, String::new(), &opt, mode, false, 0)?;
    }

    Ok(())
}

fn traverse(
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
                if is_different_fs(opt, &name, parent_inode_id, inode_id) {
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
                    if !unlink_dir(path, &name, &rel_root, visited, opt)? {
                        for entry in fs::read_dir(path)? {
                            let path = entry?.path();
                            let rel_root = concat_relative_root(&rel_root, &name);

                            if is_different_fs(opt, &rel_root, parent_inode_id, inode_id) {
                                return Ok(());
                            }

                            traverse(path.as_os_str(), rel_root, opt, mode, false, inode_id)?;
                        }
                        // The root folder is deleted last
                        traverse(path, rel_root, opt, mode, true, inode_id)?;
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
