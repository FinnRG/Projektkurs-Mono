package main

import (
	"context"
	"fmt"
	"log"
	"net"

	uploadv1 "msostream/upload/gen/go/upload/v1"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func main() {
	initMinio()
	if err := run(); err != nil {
		log.Fatal(err)
	}
}

func run() error {
	listenOn := "0.0.0.0:50051"
	listener, err := net.Listen("tcp", listenOn)
	if err != nil {
		return fmt.Errorf("failed to listen on %s: %w", listenOn, err)
	}

	server := grpc.NewServer()
	uploadv1.RegisterUploadServiceServer(server, &uploadServiceServer{})
	log.Println("Listening on", listenOn)
	if err := server.Serve(listener); err != nil {
		return fmt.Errorf("failed to serve gRPC server: %w", err)
	}

	return nil
}

type uploadServiceServer struct {
	uploadv1.UnimplementedUploadServiceServer
}

// UploadURL returns a presigned URL for the user media upload
func (s *uploadServiceServer) UploadURL(ctx context.Context, req *uploadv1.UploadURLRequest) (*uploadv1.UploadURLResponse, error) {
	presignedURL, err := PresignedURL(req.Id)
	if err != nil {
		status.Errorf(codes.Unknown, err.Error())
	}
	return &uploadv1.UploadURLResponse{Url: presignedURL}, nil
}
