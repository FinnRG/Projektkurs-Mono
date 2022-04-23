# Rust as the base image
FROM rust:1.60 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo new --bin stream
WORKDIR /stream

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/stream*
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=build /stream/target/release/stream /usr/local/bin/stream

CMD ["stream"]