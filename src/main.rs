#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::ffi::OsString;
use std::{fs, io, path};

use clap::ArgMatches;
use thiserror::Error;

use arg::{rm_options, InteractiveMode, RmOptions};

mod arg;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Operation not permitted")]
    Permission,
    #[error("Is a directory")]
    Directory,
    #[error("Directory not empty")]
    DirectoryNotEmpty,
    #[error("No such file or directory")]
    NoSuchFile,
    #[error("oh")]
    Unknown,
}

pub enum RmStatus {
    Accept,
    Declined,
    Failed(Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = rm_options().get_matches();
    let opt = RmOptions::from(&args);
    let mode = elect_interact_level(&opt, &args);

    dbg!(mode);

    // if opt == RmOptions::default() {
    //     println!("rm: missing operand");
    //     return Ok(());
    // }
    //
    // match mode {
    //     InteractiveMode::Never => {
    //         // enable jwalk
    //         todo!()
    //     }
    //
    //     InteractiveMode::Always | InteractiveMode::Once => {
    //         // sequential
    //         for path in &opt.file {
    //             let metadata = verify_metadata(path)?;
    //         }
    //     }
    // }

    todo!()
}

/// Get the last occurence of a flag and return its index
fn last_flag_occurence(indices_of: Option<clap::Indices>) -> usize {
    *indices_of
        .map(std::iter::Iterator::collect::<Vec<usize>>)
        .unwrap_or_default()
        .last()
        .unwrap_or(&0)
}

#[must_use]
pub fn elect_interact_level(opt: &RmOptions, args: &ArgMatches) -> InteractiveMode {
    let flag_always = last_flag_occurence(args.indices_of("interactive_always"));
    let flag_once = last_flag_occurence(args.indices_of("interactive_once"));
    let flag_mode = last_flag_occurence(args.indices_of("WHEN"));

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

// pub fn verify_metadata(path: &OsString) -> Result<fs::Metadata> {
//     fs::metadata(path).map_err(|err| {
//         println!("{:?}", err);
//         Error::NoSuchFile
//     })
// }
//

// pub fn prompt(opt: &RmOptions, is_dir: bool, path: &OsString) -> RmStatus {
//     let mode = opt.interactive;
//
//     if matches!(mode, InteractiveMode::Never) {
//         return RmStatus::Accept;
//     }
//     let write_protected = metadata.permissions().readonly();
//
//     if !write_protected && is_dir && !opt.dir {
//         return RmStatus::Failed(Error::Directory);
//     }
//
//     let is_empty_dir = path::PathBuf::from(path)
//         .read_dir()
//         .unwrap()
//         .next()
//         .is_none();
//
//     if !is_empty_dir && opt.dir {
//         return RmStatus::Failed(Error::DirectoryNotEmpty);
//     }
//
//     todo!()
// }
