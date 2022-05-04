package kafka

import (
	"encoding/json"
	"fmt"
	"log"
	"os"
	"os/signal"
	"sync"

	"github.com/Shopify/sarama"
)

type Video struct {
	Id          string `json:"id"`
	Title       string `json:"title"`
	Description string `json:"description"`
	Author      string `json:"author"`
	Date        string `json:"date"`
	Visbility   string `json:"visibility"`
	Status      string `json:"status"`
}

var KAFKA_URL = os.Getenv("KAFKA_URL")
var sp sarama.SyncProducer
var sperr error

var collectedVideos map[string]Video = make(map[string]Video)
var WorkChan chan Video = make(chan Video, 1000000)

// List of videos that were already processed (Not necessarily by this instance)
var processedList []string

func InitKafka() {
	if len(KAFKA_URL) == 0 {
		KAFKA_URL = "kafka:9092"
		log.Printf("KAFKA_URL set to default: %s\n", KAFKA_URL)
	}
	config := sarama.NewConfig()
	config.Producer.Return.Successes = true
	config.Producer.Return.Errors = true
	config.Producer.Retry.Max = 10
	sp, sperr = sarama.NewSyncProducer([]string{KAFKA_URL}, config)

	if sperr != nil {
		log.Fatalln(sperr)
	}
}

func CollectVideos(wg *sync.WaitGroup) {

	log.Println("Started collecting video events")
	config := sarama.NewConfig()
	consumer, err := sarama.NewConsumer([]string{KAFKA_URL}, config)
	if err != nil {
		log.Fatalln(err)
	}

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

	client, _ := sarama.NewClient([]string{KAFKA_URL}, config)
	lastOffset, _ := client.GetOffset("videos", 0, sarama.OffsetNewest)
	log.Printf("Found last offset: %d", lastOffset)
	if err != nil {
		log.Fatalln(err)
	}
	if lastOffset == 0 {
		wg.Done()
	}

ConsumerLoop:
	for {
		select {
		case msg := <-partitionConsumer.Messages():
			log.Printf("%d\n", msg.Offset)
			checkHeaders(msg)

			if msg.Offset == lastOffset-1 && lastOffset != 0 {
				// Start transcoding
				fmt.Println("Finished traversing old events")
				wg.Done()
			}
		case <-signals:
			break ConsumerLoop
		}
	}
}

func checkHeaders(msg *sarama.ConsumerMessage) {
	for _, header := range msg.Headers {
		if processedVideoHeader(header) {
			processedList = append(processedList, string(msg.Key))
		} else if uploadedVideoHeader(header) {
			collectVideo(msg)
		}
	}

}

func collectVideo(msg *sarama.ConsumerMessage) {
	var video Video
	err := json.Unmarshal(msg.Value, &video)
	if err != nil {
		log.Fatalln(err)
	}
	collectedVideos[video.Id] = video
	WorkChan <- video
}

func processedVideoHeader(header *sarama.RecordHeader) bool {
	if string(header.Key) == "type" && string(header.Value) == "Processed" {
		delete(collectedVideos, string(header.Key))
		return true
	}
	return false
}

func uploadedVideoHeader(header *sarama.RecordHeader) bool {
	log.Printf("Header: %s = %s\n", string(header.Key), string(header.Value))
	return string(header.Key) == "type" && (string(header.Value) == "Uploaded" || string(header.Value) == "Updated")
}

// Emits an event with the header type=Processed onto the video topic
func EmitVideoProcessedEvent(id string) error {
	log.Println("Started emitting VideoProcessed event")

	m, err := json.Marshal(collectedVideos[id])
	if err != nil {
		return err
	}

	typeHeader := sarama.RecordHeader{Key: []byte("type"), Value: []byte("Processed")}
	headers := []sarama.RecordHeader{typeHeader}
	msg := &sarama.ProducerMessage{Topic: "videos", Key: sarama.StringEncoder(id), Value: sarama.ByteEncoder(m), Headers: headers}

	_, _, err = sp.SendMessage(msg)
	if err != nil {
		return err
	}
	return nil
}

func IsProcessed(id string) bool {
	for _, v := range processedList {
		if v == id {
			return true
		}
	}
	return false
}

func EmitVideoEvent(id string, eventType string) error {
	log.Println("Started emitting VideoProcessed event")

	typeHeader := sarama.RecordHeader{Key: []byte("type"), Value: []byte("Processed")}
	headers := []sarama.RecordHeader{typeHeader}
	msg := &sarama.ProducerMessage{Topic: "videos", Key: sarama.StringEncoder(id), Headers: headers}

	_, _, err := sp.SendMessage(msg)
	if err != nil {
		return err
	}
	return nil
}
