# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.5 - 15/10/2022

# Added

New shred feature.
This flag (`--shred`) will perform multiple overwrite operations on this file
and introduce noise to make it's contents unrecoverable.

# Changed

### Benchmarks

Added `rmd` and `rmt` to benchmark comparison

## v0.1.3 - 05/10/2022

# Changed

### Removed `flatten` feature.

Removing this feature as this doesn't make sense to include in a tool to remove
files.

### Removed `rsync` from benchmark comparison

Rsync was causing problems in MacOS.

### Removed 'last flag wins' flag election logic

`rm` uses the last interactivity flag provided, e.g. `rm -iI` will use `-I`.
Removing this to improve initial performance, now `rmx` will elect
`InteractiveMode::Always`, `InteractiveMode::Once`, `InteractiveMode::Never` in
this order.
