use crate::arg::InteractiveMode;
use crate::core::{concat_relative_root, RmStatus, BIN_NAME};
use crate::interact;

#[must_use]
pub fn prompt(name: &str, rel_root: &str, mode: InteractiveMode) -> RmStatus {
    let message = format!(
        "{bin}: remove symbolic link '{relative_name}'?",
        bin = BIN_NAME,
        relative_name = concat_relative_root(rel_root, name)
    );

    let maybe_interact = match mode {
        InteractiveMode::Always => interact::with_message(message),
        InteractiveMode::Once | InteractiveMode::Never => Ok(true),
    };

    if let Ok(yes) = maybe_interact {
        if yes {
            return RmStatus::Accept;
        }
        return RmStatus::Declined;
    }

    RmStatus::Failed(maybe_interact.unwrap_err())
}
