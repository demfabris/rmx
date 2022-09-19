use std::ffi::OsStr;
use std::fs;

use crate::arg::{InteractiveMode, RmOptions};
use crate::core::{concat_relative_root, is_empty_dir, is_write_protected, RmStatus};
use crate::error::Error;
use crate::interact;

#[must_use]
/// # Panics
pub fn prompt(
    opt: &RmOptions,
    path: &OsStr,
    rel_root: &str,
    metadata: &fs::Metadata,
    name: &str,
    mode: InteractiveMode,
    visited: bool,
) -> RmStatus {
    let is_empty_dir = is_empty_dir(path);

    if !opt.recursive {
        if !opt.dir {
            return RmStatus::Failed(Error::IsDirectory(name.to_owned()));
        }

        if opt.dir && !is_empty_dir {
            return RmStatus::Failed(Error::DirectoryNotEmpty(name.to_owned()));
        }
    }

    let write_protected = is_write_protected(metadata);
    let descend = opt.recursive && !is_empty_dir && !visited;
    let message = format!(
        "rm: {descend_remove}{write_protected}directory '{relative_name}'?",
        descend_remove = if descend { "descend into" } else { "remove" },
        write_protected = if write_protected {
            " write-protected "
        } else {
            " "
        },
        relative_name = concat_relative_root(rel_root, name)
    );

    let maybe_interact = match mode {
        InteractiveMode::Always => {
            if (is_empty_dir && opt.dir) || opt.recursive {
                interact::with_message(message)
            } else {
                Ok(true)
            }
        }
        InteractiveMode::Once => {
            println!("unimplemented weak prompt");
            Ok(true)
        }
        InteractiveMode::Never => {
            if (opt.dir || opt.recursive) && write_protected && !opt.force {
                interact::with_message(message)
            } else {
                Ok(true)
            }
        }
    };

    if let Ok(yes) = maybe_interact {
        if yes {
            return RmStatus::Accept;
        }
        return RmStatus::Declined;
    }

    RmStatus::Failed(maybe_interact.unwrap_err())
}
