FROM golang:1.18-alpine AS build

WORKDIR /upload
COPY go.mod ./
COPY go.sum ./
RUN go mod download
COPY *.go ./
RUN CGO_ENABLED=0 GOOS=linux GIN_MODE=release go build -a -o upload .
 
FROM redhat/ubi8-minimal:8.5
WORKDIR /
COPY --from=build /upload /
CMD ["./upload"]