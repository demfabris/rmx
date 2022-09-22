use std::fs;

use crate::arg::InteractiveMode;
use crate::core::{concat_relative_root, is_write_protected, RmStatus};
use crate::interact;

#[must_use]
pub fn prompt(
    metadata: &fs::Metadata,
    name: &str,
    rel_root: &str,
    mode: InteractiveMode,
) -> RmStatus {
    let write_protected = is_write_protected(metadata);
    let empty = metadata.len() == 0;

    let message = format!(
        "rm: remove{write_protected}regular{empty}file '{relative_name}'?",
        write_protected = if write_protected {
            " write-protected "
        } else {
            " "
        },
        empty = if empty { " empty " } else { " " },
        relative_name = concat_relative_root(rel_root, name)
    );

    let maybe_interact = match mode {
        InteractiveMode::Always => interact::with_message(message),
        InteractiveMode::Once => Ok(true),
        InteractiveMode::Never => {
            if write_protected {
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
