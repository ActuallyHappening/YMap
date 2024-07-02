# YMap
A personal project to build a complete and professional iOS app using 100% Rust.

This means the developer tooling is 100% Rust, any scripting is using a Rust-implemented scripting language (i.e. `nushell`), the database is Rust-implemented (i.e. `surrealdb`), e.t.c.

The `crates/*` directory contains all the helper rust code, and the primary package (`ymap`) in the root of the workspace is the tip of the iceberg, the primary main application code.
Each crate in the `crates/*` directory is designed to solve a single task correctly, and can probably be used in your own projects. If the crate is sufficiently useful and generic,
I will publish is on `crates.io`:

## Crates ready for general use
- [`cargo-xcode-build-rs`](./crates/cargo_xcode_build_rs/) - Published on crates.io

<!-- Infinite mind mapping software

https://dev.to/wadecodez/exploring-rust-for-native-ios-game-development-2bna

## ML training and execution on device!!! WGPU!!
https://burn.dev/book/basic-workflow/training.html

## Developing
```nu
cargo install cargo-run-script

# alias of rs => run-script in .cargo/config.toml
cargo rs dev-install
```

### TODO:
- use CLI to build and run the project through xcode and yap plugin -->