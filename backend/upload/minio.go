package main

import (
	"context"
	"log"
	"sync"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

var BUCKET string = "raw"

func initMinio(wg *sync.WaitGroup) {
	location := "eu-west-1"

	minioClient, ctx := MinioClient()

	err := minioClient.MakeBucket(ctx, BUCKET, minio.MakeBucketOptions{Region: location})
	if err != nil {
		// Check to see if we already own this bucket
		exists, errBucketExists := minioClient.BucketExists(ctx, BUCKET)
		if errBucketExists == nil && exists {
			log.Printf("%s already created\n", BUCKET)
		} else {
			log.Fatalln(err)
		}
	} else {
		log.Printf("Successfully created %s\n", BUCKET)
	}
	wg.Done()
}

func MinioClient() (*minio.Client, context.Context) {
	ctx := context.Background()
	endpoint := "minio:9000"
	accessKeyID := "admin"
	secretAccessKey := "strongPassword"
	useSSL := false

	// Initialize minio client object.
	minioClient, err := minio.New(endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(accessKeyID, secretAccessKey, ""),
		Secure: useSSL,
	})
	if err != nil {
		log.Fatalln(err)
	}

	return minioClient, ctx
}
