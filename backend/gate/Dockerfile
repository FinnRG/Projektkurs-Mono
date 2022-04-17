FROM golang:1.18-alpine

RUN mkdir /gate
COPY . /gate
WORKDIR /gate

RUN go build -o gateway .

EXPOSE 8080
EXPOSE 50051

CMD [ "/gate/gateway" ]