#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::ffi::OsStr;
use std::fs;

use crate::arg::{elect_interact_level, rm_options, InteractiveMode, RmOptions};
use crate::core::{concat_relative_root, fs_entity, is_empty_dir, FsEntity, RmStatus};
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
                RmStatus::Accept => {
                    println!("execute");
                    fs::remove_file(path).expect("to remove");
                }
                RmStatus::Descend(_) | RmStatus::Declined => return Ok(()),
                RmStatus::Failed(err) => {
                    return Err(err);
                }
            }
        }

        FsEntity::Dir { metadata, name } => {
            match dir::prompt(opt, path, &rel_root, &metadata, &name, mode, visited) {
                RmStatus::Accept => {
                    println!("execute");
                    let is_empty = is_empty_dir(path);
                    if is_empty {
                        fs::remove_dir_all(path).expect("to remove");
                    }
                }
                RmStatus::Descend(folder) => {
                    println!("descend");
                    for entry in fs::read_dir(folder)? {
                        let path = entry?.path();
                        let rel_root = concat_relative_root(&rel_root, &name);
                        traverse(path.as_os_str(), rel_root, opt, mode, false)?;
                    }
                    // The root folder is deleted last
                    traverse(folder, rel_root, opt, mode, true)?;
                }
                RmStatus::Declined => return Ok(()),
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
    Ok(())
}
