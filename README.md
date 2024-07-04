# YMap
A personal project to build a complete and professional iOS app using 100% Rust.

This means the developer tooling is 100% Rust, any scripting is using a Rust-implemented scripting language (i.e. `nushell`), the database is Rust-implemented (i.e. `surrealdb`), e.t.c.

The `crates/*` directory contains all the helper rust code, and the primary package (`ymap`) in the root of the workspace is the tip of the iceberg, the primary main application code.
Each crate in the `crates/*` directory is designed to solve a single task correctly, and can probably be used in your own projects. If the crate is sufficiently useful and generic,
I will publish is on `crates.io`:

## Crates ready for general use
- [`cargo-xcode-build-rs`](./crates/cargo_xcode_build_rs/) - Published on crates.io

## Collection of Rust projects in use
- SurrealBD
- Nushell
- [Typos](https://crates.io/crates/typos)

## Compiling for WASM notes
This project naturally needs to support wasm32 targets for all crates.
This means by default tokio doesn't have multithreading, but this can be added back on a per-crate basis (can't use target dependencies in cargo workspaces which is annoying).

Also, add this to the end of your `$nu.config-path`:
```nu
$env.path = ($env.path | prepend "/opt/homebrew/opt/llvm/bin")   
```
This ensures that `clang --version` returns this:
```
~/Desktop/YMap/crates/yauth> clang --version
Homebrew clang version 18.1.8
Target: arm64-apple-darwin23.5.0
Thread model: posix
InstalledDir: /opt/homebrew/opt/llvm/bin
```
Because that seems to fix the `error: failed to build archive: 'wasm32.o': section too large` error?

Also don't forget `rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim wasm32-unknown-unknown`

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

<!-- ## Setup
`ln -s ~/.env/ymap/env.nu env.nu`

### SSH
`-f` run in background
`-N` don't execute any (remote) commands
`-T` disables interactive shells

```nu
# will factor reset everything
# and automatically import db.surql
db server reset

db connect
> info for db
``` -->