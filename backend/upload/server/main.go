package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"time"

	uploadv1 "msostream/upload/gen/go/upload/v1"

	"golang.org/x/sync/errgroup"
	"google.golang.org/grpc"

	"github.com/gin-gonic/gin"
	"github.com/minio/minio-go/v7"
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
	r.POST("/", upload)

	log.Println("Listening on", 8080)
	return r
}

func upload(c *gin.Context) {
	file, ferr := c.FormFile("file")
	if ferr != nil {
		c.AbortWithStatus(http.StatusBadRequest)
	}

	client, err := MinioClient()
	if err != nil {
		c.AbortWithStatus(http.StatusInternalServerError)
	}

	c.SaveUploadedFile(file, "test.mp4")

	client.FPutObject(context.Background(), BUCKET, "test", "test.mp4", minio.PutObjectOptions{
		ContentType: "video/mp4",
	})

	c.String(http.StatusOK, fmt.Sprintf("'%s uploaded!", file.Filename))
}

type uploadServiceServer struct {
	uploadv1.UnimplementedUploadServiceServer
}

func (s *uploadServiceServer) UploadURL(ctx context.Context, req *uploadv1.UploadURLRequest) (*uploadv1.UploadURLResponse, error) {
	return &uploadv1.UploadURLResponse{}, nil
}
