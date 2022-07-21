use clap::{ArgEnum, Parser};
use std::ffi::OsString;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// ignore nonexistent files and arguments, never prompt
    #[clap(short = 'f', long = "force")]
    force: bool,

    /// prompt before every removal
    #[clap(short = 'i')]
    interactive_weak: bool,

    /// prompt once before removing more than three files, or when removing recursively; less
    /// intrusive than -i, while still giving protection against most mistakes
    /// prompt according to WHEN: never, once (-I), or always (-i); without WHEN, prompt always
    #[clap(
        short = 'I',
        long = "interactive[=WHEN]",
        arg_enum,
        value_parser,
        default_value = "never"
    )]
    interactive: InteractiveMode,

    /// when removing a hierarchy recursively, skip any directory that is on a file system different
    /// from that of the corresponding command line argument
    #[clap(long = "one-file-system")]
    one_file_system: bool,

    /// do not remove '/' (default); with 'all', reject any command line argument on a separate
    /// device from its parent
    #[clap(long = "preserve-root[=all]")]
    preserve_root: bool,

    /// remove directories and their contents recursively, alias: -R
    #[clap(short = 'r', short_alias('R'), long = "recursive")]
    recursive: bool,

    /// remove empty directories
    #[clap(short = 'd', long = "dir")]
    dir: bool,

    /// explain what is being done
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,

    #[clap()]
    file: Vec<OsString>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ArgEnum)]
enum InteractiveMode {
    Never,
    Once,
    Always,
}

fn main() {
    let app = Args::parse();

    dbg!(app);
}
