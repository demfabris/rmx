use thiserror::Error;

use crate::core::BIN_NAME;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{}", fmt_error("Permission denied", Some(.0)))]
    PermissionDenied(String),

    #[error("{}", fmt_error("Operation not permitted", Some(.0)))]
    OperationNotPermitted(String),

    #[error("{}", fmt_error("Is a directory", Some(.0)))]
    IsDirectory(String),

    #[error("{}", fmt_error("Directory not empty", Some(.0)))]
    DirectoryNotEmpty(String),

    #[error("{}", fmt_error("No such file or directory", Some(.0)))]
    NoSuchFile(String),

    #[error("{}", fmt_error("Unknown file system entity", Some(.0)))]
    UnknownEntity(String),

    #[error("{}: failed to access system trash bin", BIN_NAME)]
    TrashBin(#[from] trash::Error),

    #[error(
        "{}: missing operand\nTry '{} --help' for more information.",
        BIN_NAME,
        BIN_NAME
    )]
    Usage,

    #[error("{}: cannot remove: {}", BIN_NAME, .0)]
    Io(#[from] std::io::Error),
}

fn fmt_error(cause: &str, maybe_name: Option<&str>) -> String {
    let name = maybe_name.map_or_else(|| String::from(""), |name| format!(" '{}'", name));
    format!(r"rmx: cannot remove{}: {}", name, cause)
}
