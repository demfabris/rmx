#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use crate::arg::{elect_interact_level, rm_options, InteractiveMode, RmOptions};
use crate::core::{Result, BIN_NAME};
use error::Error;

mod arg;
mod core;
mod dir;
mod error;
mod file;
mod interact;
mod link;
mod traverse;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}

fn run() -> Result<()> {
    let args = rm_options().get_matches();
    let opt = RmOptions::from(&args);

    // Rip mode
    if opt.rip {
        for path in &opt.file {
            traverse::walk(path)?;
        }

        return Ok(());
    }

    if opt == RmOptions::default() && !opt.force {
        return Err(Error::Usage);
    }

    let mode = elect_interact_level(&opt, &args);
    if mode == InteractiveMode::Once && (opt.file.len() > 3 || opt.recursive) {
        let message = format!(
            "{bin}: remove {count} {arguments}{recursive}?",
            bin = BIN_NAME,
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
        traverse::dfs(path, String::new(), &opt, mode, false, 0)?;
    }

    Ok(())
}
