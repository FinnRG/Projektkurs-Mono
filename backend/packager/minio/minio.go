package minio

import (
	"context"
	"log"
	"os"
	"sync"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

var S3BUCKET string = os.Getenv("S3BUCKET")
var S3ENDPOINT string = os.Getenv("S3ENDPOINT")
var S3ACCESSKEYID string = os.Getenv("S3ACCESSKEYID")
var S3ACCESSKEY string = os.Getenv("S3ACCESSKEY")
var _, S3USESSL = os.LookupEnv("S3USESSL")

func InitMinio(wg *sync.WaitGroup) {
	initEnv()
	log.Println("Initializing MinIO")
	location := "eu-west-1"

	minioClient, ctx := MinioClient()

	err := minioClient.MakeBucket(ctx, S3BUCKET, minio.MakeBucketOptions{Region: location})
	if err != nil {
		// Check to see if we already own this bucket
		exists, errBucketExists := minioClient.BucketExists(ctx, S3BUCKET)
		if errBucketExists == nil && exists {
			log.Printf("%s already created\n", S3BUCKET)
		} else {
			log.Fatalln(err)
		}
	} else {
		log.Printf("Successfully created %s\n", S3BUCKET)
	}
	wg.Done()
}

func initEnv() {
	if len(S3BUCKET) == 0 {
		S3BUCKET = "videos"
		log.Printf("S3BUCKET set to default: %s\n", S3BUCKET)
	}
	if len(S3ENDPOINT) == 0 {
		S3ENDPOINT = "minio:9000"
		log.Printf("S3ENDPOINT set to default: %s\n", S3ENDPOINT)
	}
	if len(S3ACCESSKEYID) == 0 {
		S3ACCESSKEYID = "admin"
		log.Printf("S3ACCESSKEYID set to default: %s\n", S3ACCESSKEYID)
	}
	if len(S3ACCESSKEY) == 0 {
		log.Fatalln("Error: S3ACCESSKEY not set")
	}
}

func MinioClient() (*minio.Client, context.Context) {
	ctx := context.Background()

	// Initialize minio client object.
	minioClient, err := minio.New(S3ENDPOINT, &minio.Options{
		Creds:  credentials.NewStaticV4(S3ACCESSKEYID, S3ACCESSKEY, ""),
		Secure: S3USESSL,
	})
	if err != nil {
		log.Fatalln(err)
	}

	return minioClient, ctx
}

func UploadFolder(id string, client *minio.Client, ctx context.Context) error {
	items, err := os.ReadDir("/" + id + "/output")
	if err != nil {
		return err
	}

	for _, item := range items {
		_, err := client.FPutObject(ctx, S3BUCKET, id+"/"+item.Name(), "/"+id+"/output/"+item.Name(), minio.PutObjectOptions{})
		if err != nil {
			return err
		}
	}
	return nil
}
