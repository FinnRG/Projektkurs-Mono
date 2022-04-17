FROM golang:1.18-alpine

RUN mkdir packager/
COPY . /packager
WORKDIR /packager

RUN go build -o packager ./server/

EXPOSE 8080

CMD [ "/packager/packager" ]