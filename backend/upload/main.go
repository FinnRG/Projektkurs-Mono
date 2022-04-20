package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"strconv"
	"sync"

	sarama "github.com/Shopify/sarama"
	"github.com/gin-gonic/gin"
	"github.com/minio/minio-go/v7"
)

var collectedVideos map[string]string = make(map[string]string)
var supportedFileTypes []string = []string{"video/mp4", "video/quicktime", "video/x-troff-msvideo", "video/avi", "video/msvideo", "video/x-msvideo", "video/x-flv", "video/x-ms-wmv", "video/x-matroska"}

func main() {
	var wg sync.WaitGroup
	wg.Add(2)
	go collectVideos(&wg)
	go initMinio(&wg)

	wg.Wait()
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

func collectVideos(wg *sync.WaitGroup) {
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
	partitionConsumer, err := consumer.ConsumePartition("videos", 0, sarama.OffsetOldest)
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

	client, _ := sarama.NewClient([]string{"kafka:9092"}, config)
	lastOffset, _ := client.GetOffset("videos", 0, sarama.OffsetNewest)
	log.Println("Found last offset: " + strconv.Itoa(int(lastOffset)))
	if err != nil {
		log.Fatalln(err)
	}

ConsumerLoop:
	for {
		select {
		case msg := <-partitionConsumer.Messages():
			var video Video
			err := json.Unmarshal([]byte(msg.Value), &video)

			if err != nil {
				log.Fatal(err)
			}

			for _, s := range msg.Headers {
				if string(s.Key) == "type" {
					collectedVideos[string(msg.Key)] = string(video.Visbility)
				}
			}

			if msg.Offset == lastOffset-1 {
				// Start the upload server, while still listening to events
				log.Println("Finished reading old video events")
				wg.Done()
			}
		case <-signals:
			break ConsumerLoop
		}
	}
}

func run() error {
	router := gin.Default()
	router.POST("/upload/:id", upload)
	log.Println("Starting server at 0.0.0.0:8080")
	return router.Run("0.0.0.0:8080")
}

func upload(c *gin.Context) {

	// Ensure that the user is authorized and an upload is registered for this id
	id := c.Param("id")
	if !uploadAuthorized(id) {
		c.AbortWithStatus(http.StatusUnauthorized)
	}

	file, ferr := c.FormFile("file")
	if ferr != nil {
		c.AbortWithStatus(http.StatusBadRequest)
	}

	supported, fileType := supportedFileType(c)
	if !supported {
		c.AbortWithStatus(http.StatusUnsupportedMediaType)
	}

	client, err := MinioClient()
	if err != nil {
		c.AbortWithStatus(http.StatusInternalServerError)
	}

	c.SaveUploadedFile(file, id)

	client.FPutObject(context.Background(), BUCKET, id, id, minio.PutObjectOptions{
		ContentType: fileType,
	})

	c.String(http.StatusOK, fmt.Sprintf("'%s uploaded!", file.Filename))
}

// Checks whether the supplied file is in the list of supported file types and returns the filetype
func supportedFileType(c *gin.Context) (bool, string) {
	filetype := c.ContentType()

	for _, ft := range supportedFileTypes {
		if filetype == ft {
			return true, filetype
		}
	}
	return false, ""
}

func uploadAuthorized(id string) bool {
	return collectedVideos[id] == "DRAFT"
}
