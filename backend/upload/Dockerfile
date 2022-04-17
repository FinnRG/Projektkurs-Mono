FROM golang:1.18-alpine

RUN mkdir upload/
COPY . /upload
WORKDIR /upload

RUN go build -o upload ./upload/

EXPOSE 50051

CMD [ "/upload/upload" ]