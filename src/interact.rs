use dialoguer::{theme, Confirm};

use crate::Result;

/// # Errors
///
/// Fails with I/O error if can't write to stdout
#[cfg(not(feature = "auto-interactive"))]
pub fn with_message(message: String) -> Result<bool> {
    Confirm::with_theme(&theme::ColorfulTheme::default())
        .with_prompt(message)
        .default(true)
        .show_default(true)
        .interact()
        .map_err(std::convert::Into::into)
}

#[cfg(feature = "auto-interactive")]
#[allow(clippy::needless_pass_by_value)]
pub fn with_message(message: String) -> Result<bool> {
    println!("{}", message);
    Ok(true)
}
