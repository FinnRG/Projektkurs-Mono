package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"

	sarama "github.com/Shopify/sarama"
	"github.com/gin-gonic/gin"
	"github.com/minio/minio-go/v7"
)

var collectedVideos map[string]string

func main() {
	collectVideos()
	if err := run(); err != nil {
		log.Fatal(err)
	}
}

type Video struct {
	Id          string `json:"id"`
	Title       string `json:"title"`
	Description string `json:"description"`
	Author      string `json:"author"`
	Date        string `json:"date"`
	Visbility   string `json:"visibility"`
}

func collectVideos() {
	log.Println("Started collecting video events")
	config := sarama.NewConfig()
	consumer, err := sarama.NewConsumer([]string{"kafka:9092"}, config)
	if err != nil {
		log.Fatalln(err)
	}

	defer func() {
		if err := consumer.Close(); err != nil {
			log.Fatalln(err)
		}
	}()

	log.Println("Creating PartitionConsumer")
	partitionConsumer, err := consumer.ConsumePartition("video", 0, sarama.OffsetOldest)
	if err != nil {
		log.Fatalln(err)
	}

	defer func() {
		if err := partitionConsumer.Close(); err != nil {
			log.Fatalln(err)
		}
	}()

	signals := make(chan os.Signal, 1)
	signal.Notify(signals, os.Interrupt)

	consumed := 0
ConsumerLoop:
	for {
		select {
		case msg := <-partitionConsumer.Messages():
			var video Video
			err := json.Unmarshal([]byte(msg.Value), video)
			if err != nil {
				log.Fatal(err)
			}
			log.Println(video)
			consumed++
		case <-signals:
			break ConsumerLoop
		}
	}
}

func run() error {
	router := gin.Default()
	router.POST("/upload", upload)
	return router.Run("0.0.0.0:8080")
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
