FROM bufbuild/buf:1.4.0 as buf
FROM golang:1.18-alpine AS build

WORKDIR /search
COPY go.mod ./
COPY go.sum ./
RUN go mod download
COPY *.go ./

COPY --from=buf /usr/local/bin/buf /usr/local/bin/buf
COPY buf.work.yaml buf.work.yaml
COPY buf.gen.yaml buf.gen.yaml
COPY ./proto ./proto

RUN buf generate
RUN CGO_ENABLED=0 GOOS=linux GIN_MODE=release go build -a -o search .
 
FROM redhat/ubi8-minimal:8.5
WORKDIR /
COPY --from=build /search /
CMD ["./search"]