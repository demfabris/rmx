# rmx

![Crates.io](https://img.shields.io/crates/d/rmx) ![Crates.io](https://img.shields.io/crates/l/rmx) ![Crates.io](https://img.shields.io/crates/v/rmx)

Multiplatform drop in replacement for GNU `rm` with extra features

## About

This project is a close port of GNU `rm`. The idea is extending the functionality around `rm` API and bring some niceties with improved performance, specially for large files and deeply nested directories.

## Features
- [x] :penguin: Original GNU `rm` api
- [x] :paperclip: System trash bin integration (`-t`)
- [x] :zap: Blazingly fast
- [x] :skull: Unrecoverable removal (`--shred`)
- [ ] :mag_right: More filtering options

## Benchmarks

Benches are defined [here](https://github.com/demfabris/rmx/blob/master/benches/cli.rs)

### Running

To profile how `rmx` performs on your system:

`cargo bench`

### Comparison

| remove                                    |   rmx  |  rm    |  rmt   |  rmd   |
|-------------------------------------------|--------|--------|--------|--------|
| files                                     |4.9297ms|19.991ms|10.003ms|9.2056ms|
| recursively nested folders                |4.9784ms|20.122ms|10.135ms|9.3328ms|
| multiple deeply nested folders            |4.8809ms|19.504ms|10.308ms|9.2406ms|
| multiple deeply nested folders (rip mode) |4.2580ms| -      | -      | -      |

_numbers obtained on a Alienware M15 R6, at commit: `34e1e5a2`_

`rmx` consistently scores better performance while offering the same API as GNU `rm`

## Examples

#### Deleting deeply nested directory (blazingly fast)

`rmx --rip node_modules`

#### Sending files to system trash bin

`rmx file1 file2 -t`

#### Follow symlinks (unix only)

`rmx --follow-links link`

#### Wipe a file and make it unrecoverable

`rmx --shred file`

#### Standard GNU `rm` usage

- `rmx --one-file-system -i *.txt` _handles more glob matching args, `rm` panics at ~10k+ matches`_
- `rmx --preserve-root=/home --interactive=once /home/*/*`
- `rmx --verbose -rf --no-preserve-root /`

## Installation

### Source

From [crates.io](https://crates.io/crates/rmx)

`cargo install rmx`

### Binaries

**AUR**: `yay rmx-bin`

#### Prebuilt binaries

Find all release targets [here](https://github.com/demfabris/rmx/releases)

Latest linux-musl [binary](https://github.com/demfabris/rmx/blob/master/bin/rmx)

### Pro-tip:
Put in your favorite shell rc file:
`alias rm='rmx'`

## Disclaimer

1. Do not trust this tool for automation/production usage, this is not a 1:1 port of GNU `rm` and the underlying system calls are not the same.
2. You may experience different performance results, overall `rmx` improves the computation load. I/O might still be your biggest bottleneck.
3. Making the contents of a file unrecoverable is not a fully trusted operation nowadays.
