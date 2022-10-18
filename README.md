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
- [ ] :smile: You tell me

## Benchmarks

Benches are defined [here](https://github.com/demfabris/rmx/blob/master/benches/cli.rs)

### Running

`cargo bench`

### Comparison

| remove                     | rmx | rm | rmt |
|----------------------------|-----|----|-------|
| files                      |5.7739ms|14.121ms|7.2263ms|
| recursively nested folders |5.7798ms|14.128ms|7.3677ms|
| multiple deeply nested folders      |5.2066ms|14.669ms|7.2347ms|
| multiple deeply nested folders (rip mode) |4.6359ms|14.160ms|7.5436ms|

_numbers obtained on a XPS 13 9300, at commit: `7929f6`_

## Examples

### Deleting deeply nested directory (blazingly fast)

`rmx --rip node_modules`

### Sending files to system trash bin

`rmx file1 file2 -t`

### Follow symlinks (unix only)

`rmx --follow-links link`

### Wipe a file and make it unrecoverable

`rmx --shred file`

### Standard GNU `rm` usage

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
