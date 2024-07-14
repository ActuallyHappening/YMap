# Starts blank

# Get started with a build env with Rust nightly
# FROM rustlang/rust:nightly-bullseye as builder

# If you’re using stable, use this instead
FROM rust:1-bullseye as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

RUN apt-get update -y && apt-get install -y build-essential clang

# RUN apt install build-essential

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app

COPY rust-toolchain.toml .

# for performance, forces rustup to download cargo and friends
RUN cargo --help

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# includes external dependencies that are in the parent directory
# for some dumb reason docker doesn't copy files from outside the build context, so this can't be ..
# COPY .. .
# COPY . .
# being explicit about all copies
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src/
COPY crates/ crates/
COPY assets/ assets/
COPY .cargo/ .cargo/
COPY rust-toolchain.toml .
COPY init.surql init.surql
COPY website/ website/
# doesn't copy secrets.rs

WORKDIR /app/website

# Build the app
RUN cargo leptos build --release -vv --features production

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/ymap_website /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info,ymap=debug,yauth=debug,ysurreal=debug,ymap-website=debug"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Run the server
CMD ["/app/ymap_website"]
