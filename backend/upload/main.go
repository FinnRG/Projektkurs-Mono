package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"mime/multipart"
	"net/http"
	"os"
	"os/signal"
	"strconv"
	"sync"

	sarama "github.com/Shopify/sarama"
	"github.com/gin-gonic/gin"
	"github.com/minio/minio-go/v7"
)

var collectedVideos map[string]Video = make(map[string]Video)
var supportedFileTypes []string = []string{"video/mp4", "video/quicktime", "video/x-troff-msvideo", "video/avi", "video/msvideo", "video/x-msvideo", "video/x-flv", "video/x-ms-wmv", "video/x-matroska", "video/webm", "video/ogg"}
var uploadedList []string

func main() {

	var wg sync.WaitGroup
	wg.Add(3)
	go collectVideos(&wg)
	go initMinio(&wg)
	go initSyncProducer(&wg)

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
				if createdVideoHeader(s) && notUploaded(video.Id) {
					collectedVideos[string(msg.Key)] = video
				}
				if uploadedVideoHeader(s) {
					log.Println("New Uploaded event with id: " + string(msg.Key))
					uploadedList = append(uploadedList, string(msg.Key))
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

	// Check that the user is authorized
	id := c.Param("id")
	if !uploadAuthorized(id) {
		c.AbortWithStatus(http.StatusUnauthorized)
		return
	}

	if !notUploaded(id) {
		c.AbortWithStatus(http.StatusConflict)
		return
	}

	// Check that the uploaded file is valid
	file, ferr := c.FormFile("file")
	if ferr != nil {
		c.AbortWithStatus(http.StatusBadRequest)
		return
	}

	// Check that the file type is supported
	supported, fileType := supportedFileType(file)
	if !supported {
		c.AbortWithStatus(http.StatusUnsupportedMediaType)
		return
	}

	client, _ := MinioClient()

	// Save file to local directory
	c.SaveUploadedFile(file, id)

	// Upload file to MinIO
	client.FPutObject(context.Background(), BUCKET, id, id, minio.PutObjectOptions{
		ContentType: fileType,
	})

	// Emit VideoUploaded event to kafka
	err := emitVideoUploadedEvent(id)
	if err != nil {
		c.AbortWithStatus(http.StatusInternalServerError)
		// Remove temp file
		if err = os.Remove(id); err != nil {
			log.Println(err)
		}
		return
	}

	uploadedList = append(uploadedList, id)
	c.Status(http.StatusOK)
	c.String(http.StatusOK, fmt.Sprintf("'%s uploaded!", file.Filename))
}

func createdVideoHeader(header *sarama.RecordHeader) bool {
	return string(header.Key) == "type" && (string(header.Value) == "Created" || string(header.Value) == "Updated")
}

func uploadedVideoHeader(header *sarama.RecordHeader) bool {
	return string(header.Key) == "type" && string(header.Value) == "Uploaded"
}

// Checks whether a video with this id was already uploaded to prevent multiple uploads
func notUploaded(id string) bool {
	for _, pid := range uploadedList {
		if pid == id {
			return false
		}
	}
	return true
}

// Checks whether the supplied file is in the list of supported file types and returns the filetype
func supportedFileType(ferr *multipart.FileHeader) (bool, string) {
	filetype := ferr.Header.Get("Content-Type")

	for _, ft := range supportedFileTypes {
		if filetype == ft {
			return true, filetype
		}
	}
	return false, ""
}

func uploadAuthorized(id string) bool {
	return collectedVideos[id].Visbility == "DRAFT"
}
