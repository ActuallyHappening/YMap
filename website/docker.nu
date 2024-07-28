# Builds from the parent directory
# docker build --progress plain -f Dockerfile ..
# can be `--progress plain` if not logging to satisfaction, or `auto`

# TODO: setup a docker container with incremental compilation

cd ..
RUST_LOG="trace" cargo cross build --package=ymap-website --bin=ymap-website --release --no-default-features --features=ssr,production --target x86_64-unknown-linux-gnu --manifest-path ./website/Cargo.toml -v
# cargo build --package=ymap-website --bin=ymap-website --release --no-default-features --features=ssr,production --target x86_64-unknown-linux-gnu
