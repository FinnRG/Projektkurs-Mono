FROM bufbuild/buf:1.3.1 as buf
FROM rust:1.60.0-buster as build

ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/auth_service
COPY --from=buf /usr/local/bin/buf /usr/local/bin/buf
COPY . .

RUN apt update && apt upgrade -y && apt install -y cmake
RUN cargo install --bin auth_service --path .


FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/auth_service /usr/local/bin/auth_service

EXPOSE 8080
CMD [ "auth_service" ]
