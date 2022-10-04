# rmx

![Crates.io](https://img.shields.io/crates/d/rmx) ![Crates.io](https://img.shields.io/crates/l/rmx) ![Crates.io](https://img.shields.io/crates/v/rmx)

Multiplatform drop in replacement for GNU `rm` with extra features

## About

This project is a close port of GNU `rm`. The idea is extending the functionality around `rm` API and bring some niceties with improved performance, specially for large files and deeply nested directories.

## Features
- [x] :penguin: Original GNU `rm` api
- [x] :paperclip: System trash bin integration (`-t`)
- [x] :zap: Blazingly fast
- [ ] :mag_right: More filtering options

## Benchmarks

Benches are defined [here](https://github.com/demfabris/rmx/blob/master/benches/cli.rs)

### Running

`cargo bench`

### Comparison

| remove                     | rmx | rm | rsync |
|----------------------------|-----|----|-------|
| files                      |5.6285ms|14.201ms|786.72ms|
| recursively nested folders |6.1517ms|14.732ms|714.86ms|
| multiple deeply nested folders      |6.3199ms|14.624ms|294.71ms|
| multiple deeply nested folders (rip mode) |4.5762ms|14.079ms|274.99ms|

_numbers obtained on a XPS 13 9300, at commit: `cace6812`_

## Examples

### Deleting deeply nested directory

`rmx --rip node_modules`

### Flattening a directory at depth = 1

#### Before
```
dir
├── dir2
│   ├── dir3
│   ├── dir4
│   │   ├── file1
│   │   └── file5
│   ├── file1
│   ├── file3
│   └── file4
├── file1
└── file2
```

`rmx --flatten 1 dir`

#### After
```
dir
├── dir2
│   ├── dir4
│   │   └── file1  # name conflicts are skipped
│   ├── file1
│   └── file5
├── file1
├── file2
├── file3
└── file4
```

## Installation

Currently only obtainable via [crates.io](https://crates.io/crates/rmx) and this repo.

`cargo install rmx`

### Pro-tip:
Put in your favorite Shell rc file:
`alias rm='rmx'`

## Disclaimer

1. Do not trust this tool for automation/production usage, this is not a 1:1 port of GNU `rm` and the underlying system calls are not the same.
2. You may experience different performance results, overall `rmx` improves the computation load. I/O might still be your biggest bottleneck.
