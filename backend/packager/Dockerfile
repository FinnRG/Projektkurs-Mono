FROM bufbuild/buf:1.4.0 as buf
FROM golang:1.18-alpine AS build

WORKDIR /usr/local/go/src/msostream/packager
COPY go.mod ./
COPY go.sum ./
RUN go mod download
COPY ./tasks ./tasks
COPY ./transcode ./transcode
COPY ./kafka ./kafka
COPY ./minio ./minio
COPY *.go ./

COPY --from=buf /usr/local/bin/buf /usr/local/bin/buf
COPY buf.work.yaml buf.work.yaml
COPY buf.gen.yaml buf.gen.yaml
COPY ./proto ./proto

RUN buf generate
RUN CGO_ENABLED=0 GOOS=linux GIN_MODE=release go build -a -o packager .
 
FROM amazon/aws-cli AS aws
FROM google/cloud-sdk:slim
WORKDIR /

RUN apt update -y && apt install -y python3-pip
RUN pip3 install --user --upgrade shaka-streamer shaka-streamer-binaries
RUN mv /root/.local/bin/shaka-streamer /bin/
COPY ./conf /conf

COPY --from=build /usr/local/go/src/msostream/packager /
CMD ["./packager"]