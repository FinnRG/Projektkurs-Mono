FROM golang:1.18-alpine

RUN mkdir upload/
COPY . /upload
WORKDIR /upload

RUN go build -o upload ./server/

EXPOSE 8080

CMD [ "/upload/upload" ]