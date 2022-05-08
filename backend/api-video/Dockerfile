FROM bufbuild/buf:1.4.0 as buf
FROM rust:1.60-bullseye as build

# create a new empty shell project
RUN USER=root cargo new --bin api-video
WORKDIR /api-video

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN apt-get update && apt-get upgrade -y && apt-get install -y cmake
RUN mv src/main.rs src/service.rs
RUN cargo build --release
RUN rm src/*.rs

COPY --from=buf /usr/local/bin/buf /usr/local/bin/buf
COPY buf.work.yaml buf.work.yaml
COPY buf.gen.yaml buf.gen.yaml
COPY ./build.rs ./build.rs
COPY ./proto ./proto
COPY ./migrations ./migrations
COPY ./src ./src

RUN rm ./target/release/deps/video_api*
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update
RUN apt-get install -y libpq-dev
COPY --from=build /api-video/target/release/video_api .

EXPOSE 8080
CMD ["./video_api"]
