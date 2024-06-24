# DEPRECIATED use [cargo-xcode-build-rs](https://crates.io/crates/cargo-xcode-build-rs) instead
This name can be reused if desired
<!-- # XCode Build RS
A 100% Rust wrapper around the iOS XCode build script found in the `bevy` examples.

Benefits of using a Rust script over a `sh` (or `nu`):
- Can be updated since the binary is distributed using `crates.io`
- Allows for global collaboration, e.g. I find a case where I need to `sudo xcode-select --install` and that suggestion can be added to the error message for everyone to benefit from
- Can handle more complex logic and using libraries easier, e.g. checks that the `cc` compiler is on `$PATH`
- Logs everything using human-understandable languages (rather than `set -exu`)

## Installation
```nu
cargo install cargo-xcode-build-rs
```

This is how I used it in my XCode project, which was copied from the `bevy` mobile example:
![Example usage in XCode](docs/xcode-usage.png)

## Usage
Use `cargo xcode-build-rs --manifest-dir . xcode` in the actual xcode script.
Use `cargo xcode-build-rs --manifest-dir . --colour test` to begin a test iOS simulator build, which shouldn't be necessary normally.

## Configuration
To compiles projects for iOS with special Cargo features enabled, add a `package.metadata.xcode-build-rs` section to your `Cargo.toml` file. For example:
```toml
[package.metadata.xcode-build-rs.ios]
## As an example, when set to true enabled default features
## which is already the default.
## Set to false to disable default features
## See Cargo's docs: https://doc.rust-lang.org/cargo/reference/features.html#dependency-features
default-features = true
## What features to enable
features = ["ios"]
```

### Help message `cargo xcode-build-rs --help`
```

```

## Acknowledgements
Based on the bevy mobile example script `build_rust_deps.sh` [here](https://github.com/bevyengine/bevy/blob/main/examples/mobile/build_rust_deps.sh).
That script is also based on the mozilla script [here](https://github.com/mozilla/glean/blob/main/build-scripts/xc-universal-binary.sh)

## Developing
`git clone https://github.com/ActuallyHappening/YMap.git`
`cd crates/ymap`
`cargo is` to install this locally on your system.
`cargo rt` to run the CLI in test mode locally on your system. -->