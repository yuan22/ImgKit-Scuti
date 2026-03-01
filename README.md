<div align="center">

# ImgKit

[![English](https://img.shields.io/badge/English-red)](README.md)
[![简体中文](https://img.shields.io/badge/简体中文-blue)](.github/docs/README.zh-CN.md)

A CLI toolkit for Android image unpacking and repacking.

</div>

## Quick Navigation

- [Supported Unpack Formats](#supported-unpack-formats)
- [Supported Pack Types](#supported-pack-types)
- [Why ImgKit](#why-imgkit)
- [Quick Start](#quick-start)
- [Command Overview](#command-overview)
- [Unpack Usage](#unpack-usage)
- [Pack Usage](#pack-usage)
- [Common Options](#common-options)

## Supported Unpack Formats

`imgkit unpack` auto-detects input image type and supports:

| Type | Unpack Support | Notes |
|---|---|---|
| EXT4 | Yes | Direct detection and extraction |
| F2FS | Yes | Direct detection and extraction |
| EROFS | Yes | Direct detection and extraction |
| Super (Android LP) | Yes | Partition-level extraction |
| Android Sparse image | Yes | Auto-converts sparse then detects filesystem |

## Supported Pack Types

`imgkit pack` currently supports:

- `super`
- `ext4`
- `f2fs`
- `erofs`

Sparse output is supported in applicable scenarios.

## Why ImgKit

- One CLI for multiple formats, no tool switching.
- Auto detection for unpack flow.
- Complete pack flow for `ext4`/`f2fs`/`erofs` and `super`.
- Android metadata integration with `file_contexts` and `fs_config`.
- Script-friendly command surface for CI pipelines.

## Quick Start

```bash
cargo build --release
./target/release/imgkit --help
```

## Command Overview

```bash
imgkit <SUBCOMMAND> [OPTIONS]
```

Available subcommands:

- `unpack`: extract image contents to directory
- `pack`: build image from directory or partition images

Show detailed help:

```bash
imgkit --help
imgkit unpack --help
imgkit pack --help
```

## Unpack Usage

```bash
imgkit unpack -i <INPUT_IMAGE> -o <OUTPUT_DIR> [OPTIONS]
```

Examples:

```bash
# Basic unpack
imgkit unpack -i system.img -o out/system

# Clean output directory before unpack
imgkit unpack -i system.img -o out/system -c

# Set log level (0-3)
imgkit unpack -i super.img -o out/super -l 2
```

## Pack Usage

```bash
imgkit pack --type <super|ext4|f2fs|erofs> [OPTIONS]
```

### Pack super

```bash
imgkit pack --type super -o super.img -d auto \
  -g main:8589934592 \
  -p system:readonly:2147483648:main \
  -p vendor:readonly:524288000:main \
  -i system=system.img \
  -i vendor=vendor.img \
  -S
```

### Pack ext4

```bash
imgkit pack --type ext4 -s system/ -o system.img -z 2147483648
```

### Pack f2fs

```bash
imgkit pack --type f2fs -s system/ -o system.img -z 2147483648 --readonly
```

### Pack erofs

```bash
imgkit pack --type erofs -s system/ -o system.img --compress lz4hc --compress-level 9
```

## Common Options

- `-l, --level <0-3>`: log level (default `1`)
- `-S, --sparse`: output sparse image when supported
- `--file-contexts <FILE>`: SELinux context file
- `--fs-config <FILE>`: file permission config
