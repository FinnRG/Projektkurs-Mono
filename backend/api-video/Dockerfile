FROM bufbuild/buf:1.4.0 as buf
FROM rust:1.60-bullseye as build

# create a new empty shell project
RUN USER=root cargo new --bin api-video
WORKDIR /api-video

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN apt-get update && apt-get upgrade -y && apt-get install -y cmake
RUN mv src/main.rs src/service.rs
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree, build file and buf
COPY --from=buf /usr/local/bin/buf /usr/local/bin/buf
COPY buf.work.yaml buf.work.yaml
COPY buf.gen.yaml buf.gen.yaml
COPY ./build.rs ./build.rs
COPY ./proto ./proto
COPY ./src ./src
RUN buf generate

# build for release
RUN rm ./target/release/deps/video_api*
RUN cargo build --release

# our final base
FROM debian:bullseye-slim

# copy the build artifact from the build stage
COPY --from=build /api-video/target/release/video_api .

EXPOSE 8080
# set the startup command to run your binary
CMD ["./video_api"]
