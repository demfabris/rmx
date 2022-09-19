use thiserror::Error;

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

    #[error("rmd: missing operand\nTry 'rmd --help' for more information.")]
    Usage,

    #[error("rmd: cannot remove: {}", .0)]
    Io(#[from] std::io::Error),
}

fn fmt_error(cause: &str, maybe_name: Option<&str>) -> String {
    let name = maybe_name.map_or_else(|| String::from(""), |name| format!(" '{}'", name));
    format!(r"rmd: cannot remove{}: {}", name, cause)
}
