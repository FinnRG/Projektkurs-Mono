FROM golang:1.18-alpine AS build

WORKDIR /usr/local/go/src/msostream/packager
COPY go.mod ./
COPY go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 GOOS=linux GIN_MODE=release go build -a -o packager .
 
FROM amazon/aws-cli AS aws
FROM google/cloud-sdk:slim
WORKDIR /

RUN apt update -y && apt install -y python3-pip
RUN pip3 install --user --upgrade shaka-streamer shaka-streamer-binaries
RUN mv /root/.local/bin/shaka-streamer /bin/
COPY ./conf /conf

# RUN pip3 install awscli --upgrade --user
# RUN mv /root/.local/bin/aws /bin/
# RUN aws configure set aws_access_key_id admin
# RUN aws configure set aws_secret_access_key strongPassword
# RUN aws configure set default.region eu-west-1

COPY --from=build /usr/local/go/src/msostream/packager /
CMD ["./packager"]