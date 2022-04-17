package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"time"

	uploadv1 "msostream/packager/gen/go/upload/v1"

	"golang.org/x/sync/errgroup"
	"google.golang.org/grpc"

	"github.com/gin-gonic/gin"
)

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}

var (
	g errgroup.Group
)

func run() error {

	grpc := &http.Server{
		Addr:         ":50051",
		Handler:      gRPCServer(),
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
	}

	http := &http.Server{
		Addr:         ":8080",
		Handler:      HTTPServer(),
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
	}

	g.Go(func() error {
		return grpc.ListenAndServe()
	})

	g.Go(func() error {
		return http.ListenAndServe()
	})

	if err := g.Wait(); err != nil {
		log.Fatal(err)
	}

	return nil
}

func gRPCServer() http.Handler {
	server := grpc.NewServer()
	uploadv1.RegisterUploadServiceServer(server, &uploadServiceServer{})
	return server
}

func HTTPServer() http.Handler {
	r := gin.Default()
	r.MaxMultipartMemory = 1000 << 20
	r.POST("/upload", upload)

	log.Println("Listening on", 8080)
	return r
}

func upload(c *gin.Context) {
	file, _ := c.FormFile("file")

	c.SaveUploadedFile(file, "test.mp4")

	c.String(http.StatusOK, fmt.Sprintf("'%s uploaded!", file.Filename))
}

type uploadServiceServer struct {
	uploadv1.UnimplementedUploadServiceServer
}

func (s *uploadServiceServer) UploadURL(ctx context.Context, req *uploadv1.UploadURLRequest) (*uploadv1.UploadURLResponse, error) {
	return &uploadv1.UploadURLResponse{}, nil
}
