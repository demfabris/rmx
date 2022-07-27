use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Operation not permitted")]
    Permission,
    #[error("Is a directory")]
    Directory,
    #[error("Directory not empty")]
    DirectoryNotEmpty,
    #[error("No such file or directory")]
    NoSuchFile(String),
    #[error("Unknown file system entity")]
    UnknownEntity(String),
    #[error("fatal error")]
    Io(#[from] std::io::Error),
}