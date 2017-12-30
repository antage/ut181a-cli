# ut181a-cli

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![Build Status](https://travis-ci.org/antage/ut181a-cli.svg?branch=master)](https://travis-ci.org/antage/ut181a-cli) [![Build status](https://ci.appveyor.com/api/projects/status/rgkw2i08tfdh4x1a?svg=true)](https://ci.appveyor.com/project/antage/ut181a-cli)

Uni-T UT181A digital multimeter (DMM) remote contorl command-line interface.
It supports USB connection only.

## Installing

You need Rust programming language toolchain to compile `ut181a-cli`.
Recommended way to install Rust is `rustup` tool.
Follow instructions at https://www.rustup.rs/.

### Linux

```
$ sudo apt-get install libudev-dev libhidapi-dev
$ cargo install ut181a-cli
```

### Windows

You can download compiled binaries at [releases](https://github.com/antage/ut181a-cli/releases) page.

Requirements for compiling from the source code:

* Rust target `*-pc-windows-msvc`: VisualStudio 2015.
* Rust target `*-pc-windows-gnu`: MinGW

```
$ cargo install ut181a-cli --features build
```

## Usage

```
$ # Connect DMM to PC.
$ # Turn on DMM, press 'F4' ('SETUP') and set 'COMMUNICATION' to 'ON' state.
$ # Set mVDC mode for example.
$ ut181a-cli list-devices
Found DMM at path '0001:0004:00'.
$ ut181a-cli read once
Mode: mVDC [] [AUTO]
Range: -600...600 mV
1.74 mVDC
FAST: -104.61425 mVDC
```

Run `ut181a-cli help` to see other commands.

## License

This software licensed under the following:

* MIT License ([LICENSE](LICENSE) or https://opensource.org/license/MIT).
