use std::borrow::ToOwned;
use std::ffi::OsString;

use clap::builder::PossibleValuesParser;
use clap::{
    crate_authors, crate_description, crate_version, Arg, ArgAction, ArgMatches, Command, ValueHint,
};

pub fn rm_options() -> Command<'static> {
    Command::new("rmd")
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::new("force")
                .help("ignore nonexistent files and arguments, never prompt")
                .short('f')
        )
        .arg(
            Arg::new("interactive_always")
                .help("prompt before every removal")
                .short('i')
        )
        .arg(
            Arg::new("interactive_once")
                .help("prompt once before removing more than three files, or when removing recursively; less
intrusive than -i, while still giving protection against most mistakes")
                .short('I')
        )
        .arg(
            Arg::new("interactive")
                .help("prompt according to WHEN: never, once (-I), or always (-i); without WHEN, prompt always")
                .long("interactive")
                .takes_value(true)
                .value_parser(PossibleValuesParser::new(vec!["never", "once", "always"]))
                .id("WHEN")
        )
        .arg(
            Arg::new("one_file_system")
                .help("when removing a hierarchy recursively, skip any directory that is on a file system different
from that of the corresponding command line argument")
                .long("one-file-system")
        )
        .arg(
            Arg::new("preserve_root")
                .help("do not remove '/' (default); with 'all', reject any command line argument on a separate
device from its parent")
                .long("preserve-root")
                .takes_value(true)
                .id("all")
                .value_hint(ValueHint::DirPath)
        )
        .arg(
            Arg::new("recursive")
                .help("remove directories and their contents recursively")
                .long("recursive")
                .short('r')
                .short_alias('R')
        )
        .arg(
            Arg::new("dir")
                .help("remove empty directories")
                .long("dir")
                .short('d')
        )
        .arg(
            Arg::new("verbose")
                .help("explain what is being done")
                .long("verbose")
                .short('v')
        )
        .arg(
            Arg::new("FILE")
                .allow_invalid_utf8(true)
                .takes_value(true)
                .value_hint(ValueHint::FilePath)
                .multiple_values(true)
        )
}

#[derive(Debug, Default, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct RmOptions {
    pub force: bool,
    pub interactive_always: bool,
    pub interactive_once: bool,
    pub interactive: InteractiveMode,
    pub one_file_system: bool,
    pub preserve_root: bool,
    pub recursive: bool,
    pub dir: bool,
    pub verbose: bool,
    pub file: Vec<OsString>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InteractiveMode {
    Never,
    Once,
    Always,
}

impl Default for InteractiveMode {
    fn default() -> Self {
        Self::Never
    }
}

impl From<&ArgMatches> for RmOptions {
    fn from(args: &ArgMatches) -> Self {
        Self {
            force: args.is_present("force"),
            interactive_always: args.is_present("interactive_always"),
            interactive_once: args.is_present("interactive_once"),
            interactive: {
                match args.value_of("WHEN") {
                    Some("always") => InteractiveMode::Always,
                    Some("once") => InteractiveMode::Once,
                    _ => InteractiveMode::Never,
                }
            },
            one_file_system: args.is_present("one_file_system"),
            preserve_root: args.is_present("all"),
            recursive: args.is_present("recursive"),
            dir: args.is_present("dir"),
            verbose: args.is_present("verbose"),
            file: args
                .get_many::<OsString>("FILE")
                .map(|t| t.map(ToOwned::to_owned).collect())
                .unwrap_or_default(),
        }
    }
}
