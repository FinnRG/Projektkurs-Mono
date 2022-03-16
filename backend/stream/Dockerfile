# Rust as the base image
FROM rust:1.57.0-slim-buster as build

# Create a new empty shell project
RUN USER=root cargo new --bin stream
WORKDIR /stream
RUN apt-get update && apt-get install -y pkg-config libssl-dev openssl

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/stream*
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
RUN apt-get update
RUN apt-get install -y ca-certificates cmake musl-tools libssl-dev pkg-config openssl
COPY --from=build /stream/target/release/stream /usr/src/stream
COPY .env .env

# Run the binary
EXPOSE 80
CMD ["/usr/src/stream"]