#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::ffi::OsStr;
use std::fs;

use crate::arg::{elect_interact_level, rm_options, InteractiveMode, RmOptions};
use crate::core::{concat_relative_root, fs_entity, unlink_dir, unlink_file, FsEntity, RmStatus};
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

    for path in &opt.file {
        traverse(path, String::new(), &opt, mode, false)?;
    }

    Ok(())
}

fn traverse(
    path: &OsStr,
    rel_root: String,
    opt: &RmOptions,
    mode: InteractiveMode,
    visited: bool,
) -> Result<()> {
    let ent = fs_entity(path)?;
    match ent {
        FsEntity::File { metadata, name } => {
            match file::prompt(&metadata, &name, &rel_root, mode) {
                RmStatus::Accept => unlink_file(path, &name, &rel_root, opt)?,
                RmStatus::Declined => return Ok(()),
                RmStatus::Failed(err) => return Err(err),
            }
        }

        FsEntity::Dir { metadata, name } => {
            match dir::prompt(opt, path, &rel_root, &metadata, &name, mode, visited) {
                RmStatus::Accept => {
                    if !unlink_dir(path, &name, &rel_root, visited, opt)? {
                        for entry in fs::read_dir(path)? {
                            let path = entry?.path();
                            let rel_root = concat_relative_root(&rel_root, &name);
                            traverse(path.as_os_str(), rel_root, opt, mode, false)?;
                        }
                        // The root folder is deleted last
                        traverse(path, rel_root, opt, mode, true)?;
                    }
                }
                RmStatus::Declined => return Ok(()),
                RmStatus::Failed(err) => return Err(err),
            }
        }

        FsEntity::Symlink { metadata, name } => {
            println!("{:?} {:?}", metadata, name);
            todo!()
        }
    }
    Ok(())
}
