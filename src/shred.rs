use std::borrow::ToOwned;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};

use rand::Rng;
use zeroize::Zeroize;

use crate::Result;

const DEFAULT_SHRED_ITERATIONS: usize = 1000;

pub struct Shredder {
    path: PathBuf,
    name: OsString,
    size_range: Range<usize>,
}

#[allow(clippy::cast_possible_truncation)]
pub fn shred(path: &OsStr) -> Result<()> {
    let size = fs::metadata(path)?.len() as usize;
    if size == 0 {
        fs::remove_file(&path)?;
        return Ok(());
    }

    Shredder::new(path, size).run()
}

impl Shredder {
    pub fn new(path: &OsStr, size: usize) -> Self {
        let size_range = (size - size / 2)..(size + size / 2);
        let name = Path::new(path)
            .file_name()
            .map(ToOwned::to_owned)
            .unwrap_or_default();

        Self {
            path: PathBuf::from(path),
            name,
            size_range,
        }
    }

    fn run(&mut self) -> Result<()> {
        let instruction: usize = rand::thread_rng().gen_range(1..=3000);

        for _ in 1..=DEFAULT_SHRED_ITERATIONS {
            match instruction {
                0..=1000 => self.noise()?,
                1001..=2000 => self.write()?,
                2001..=3000 => self.rename()?,
                _ => (),
            }
        }

        fs::remove_file(&self.path)?;

        Ok(())
    }

    fn noise(&self) -> Result<()> {
        let size: usize = rand::thread_rng().gen_range(self.size_range.clone());
        let mut bytes: Vec<u8> = (1..=size).map(|_| rand::random::<u8>()).collect();
        let name: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        fs::write(&name, &bytes)?;
        fs::remove_file(&name)?;

        bytes.zeroize();

        Ok(())
    }

    fn write(&self) -> Result<()> {
        let size: usize = rand::thread_rng().gen_range(self.size_range.clone());
        let mut bytes: Vec<u8> = (1..=size).map(|_| rand::random::<u8>()).collect();

        fs::write(&self.path, &bytes)?;

        bytes.zeroize();

        Ok(())
    }

    fn rename(&mut self) -> Result<()> {
        let name: OsString = OsString::from(
            rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(30)
                .map(char::from)
                .collect::<String>(),
        );
        let parent = self
            .path
            .clone()
            .parent()
            .map(ToOwned::to_owned)
            .unwrap_or_default();
        let path = parent.join(&name);

        fs::rename(&self.path, &path)?;
        self.path = path;
        self.name = name;

        Ok(())
    }
}
