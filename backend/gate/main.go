package main

import (
	"context"
	"log"
	"net"
	"net/http"

	uploadv1 "msostream/gate/gen/go/upload/v1"
	usersv1 "msostream/gate/gen/go/users/v1"
	videosv1 "msostream/gate/gen/go/videos/v1"

	"github.com/felixge/httpsnoop"
	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
)

func withLogger(handler http.Handler) http.Handler {
	return http.HandlerFunc(func(writer http.ResponseWriter, request *http.Request) {
		m := httpsnoop.CaptureMetrics(handler, writer, request)
		log.Printf("http[%d]-- %s -- %s\n", m.Code, m.Duration, request.URL.Path)
	})
}

func main() {
	// creating mux for gRPC gateway. This will multiplex or route request different gRPC service
	mux := runtime.NewServeMux()
	// setting up a dial up for gRPC service by specifying endpoint/target url
	if err := videosv1.RegisterVideoServiceHandlerFromEndpoint(context.Background(), mux, "api-video:8080", []grpc.DialOption{grpc.WithInsecure()}); err != nil {
		log.Fatal(err)
	}
	if err := uploadv1.RegisterUploadServiceHandlerFromEndpoint(context.Background(), mux, "upload:50051", []grpc.DialOption{grpc.WithInsecure()}); err != nil {
		log.Fatal(err)
	}
	if err := usersv1.RegisterUserServiceHandlerFromEndpoint(context.Background(), mux, "auth:8080", []grpc.DialOption{grpc.WithInsecure()}); err != nil {
		log.Fatal(err)
	}
	// Creating a normal HTTP server
	server := http.Server{
		Handler: withLogger(mux),
	}

	// creating a listener for server
	l, err := net.Listen("tcp", ":8080")
	if err != nil {
		log.Fatal(err)
	}
	// start server
	err = server.Serve(l)
	if err != nil {
		log.Fatal(err)
	}
}
