# XCode Build
A 100% Rust wrapper around the iOS XCode build script found in the `bevy` examples.

Benefits of using a Rust script over a `sh` (or `nu`):
- Can be updated since the binary is distributed using `crates.io`
- Allows for global collaboration, e.g. I find a case where I need to `sudo xcode-select --install` and that suggestion can be added to the error message for everyone to benefit from
- Can handle more complex logic and using libraries easier, e.g. checks that the `cc` compiler is on `$PATH`
- Logs everything using human-understandable languages (rather than `set -exu`)

## Installation

## Usage
Use `xcode-build-rs xcode` in the actual xcode script.
Use `xcode-build-rs --colour test` to begin a test iOS simulator build, which shouldn't be necessary normally.

### Help message `xcode-build-rs --help`
```
Build script for XCode when compiling rust for iOS

Usage: xcode-build-rs [OPTIONS] <COMMAND>

Commands:
  xcode  
  test   
  help   Print this message or the help of the given subcommand(s)

Options:
      --colour   By default, doesn't display colour because this can be annoying in the XCode terminal
  -h, --help     Print help
  -V, --version  Print version
```

## Acknowledgements
Based on the bevy mobile example script `build_rust_deps.sh` [here](https://github.com/bevyengine/bevy/blob/main/examples/mobile/build_rust_deps.sh).
That script is also based on the mozilla script [here](https://github.com/mozilla/glean/blob/main/build-scripts/xc-universal-binary.sh)

## Developing
`cargo is` to install this locally on your system.
`cargo rt` to run the CLI in test mode locally on your system.