FROM golang:1.18-alpine

RUN mkdir /gate
COPY . /gate
WORKDIR /gate

RUN go build -o gateway .

EXPOSE 8000

CMD [ "/gate/gateway" ]