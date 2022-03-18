# Rust as the base image
FROM rust:1.57.0-slim-buster as build

# Create a new empty shell project
COPY ./lib ./lib
RUN USER=root cargo new --bin backend
WORKDIR /backend
RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev ffmpeg libpq-dev

# Copy our manifests
COPY ./monolith/Cargo.lock ./Cargo.lock
COPY ./monolith/Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./monolith/Rocket.toml.docker ./Rocket.toml
COPY ./monolith/src ./src

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
COPY ./monolith/.env.docker .env

# Run the binary
EXPOSE 80
ENV ROCKET_PROFILE=release
ENV ROCKET_PORT=80
ENV ROCKET_ADDRESS=0.0.0.0 
CMD ["/usr/src/backend"]