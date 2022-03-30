# Rust as the base image
FROM rust:1.57.0-slim-buster as build

# Create a new empty shell project
RUN USER=root cargo new --bin auth
WORKDIR /auth
RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev libpq-dev

# Copy relevant files
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./migrations/ ./migrations/
RUN mv src/main.rs src/service.rs

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/auth*
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
RUN apt-get update
RUN apt-get install -y libpq-dev libssl-dev
COPY --from=build /auth/target/release/auth_service /usr/src/auth_service

# Run the binary
EXPOSE 50051
CMD ["/usr/src/auth_service"]