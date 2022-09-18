#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use crate::arg::{elect_interact_level, rm_options, RmOptions};
use crate::core::{fs_entity, FsEntity, RmStatus};
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

    // let mut dir_st = vec![String::new(); 10];
    for path in &opt.file {
        let ent = fs_entity(path)?;
        match ent {
            FsEntity::File { metadata, name } => match file::prompt(&metadata, &name, mode) {
                RmStatus::Accept => {
                    println!("execute");
                }
                RmStatus::Descend(_) | RmStatus::Declined => continue,
                RmStatus::Failed(err) => {
                    return Err(err);
                }
            },

            FsEntity::Dir { metadata, name } => {
                match dir::prompt(&opt, path, &metadata, &name, mode) {
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
