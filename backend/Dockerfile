# Rust as the base image
FROM rust:1.57.0-slim-buster as build

# Create a new empty shell project
RUN USER=root cargo new --bin backend
WORKDIR /backend
RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev ffmpeg libpq-dev

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./postgres ./postgres

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./Rocket.toml.docker ./Rocket.toml
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/backend*
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
RUN apt-get update
RUN apt-get install -y ffmpeg libpq-dev libssl-dev
COPY --from=build /backend/target/release/backend /usr/src/backend
COPY --from=build /backend/Rocket.toml Rocket.toml
COPY .env.docker .env

# Run the binary
EXPOSE 8000
CMD ["/usr/src/backend"]