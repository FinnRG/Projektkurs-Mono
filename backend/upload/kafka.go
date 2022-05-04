package main

import (
	"encoding/json"
	"log"
	"sync"

	sarama "github.com/Shopify/sarama"
)

var sp sarama.SyncProducer
var sperr error

func initSyncProducer(wg *sync.WaitGroup) {
	config := sarama.NewConfig()
	config.Producer.Return.Successes = true
	config.Producer.Return.Errors = true
	// High due to the importance of the event
	config.Producer.Retry.Max = 10
	sp, sperr = sarama.NewSyncProducer([]string{"kafka:9092"}, config)

	if sperr != nil {
		log.Fatalln(sperr)
	}
	wg.Done()
}

func emitVideoUploadedEvent(id string) error {
	video := collectedVideos[id]
	video.Status = "STATUS_PROCESSED"
	m, err := json.Marshal(video)
	if err != nil {
		return err
	}
	typeHeader := sarama.RecordHeader{Key: []byte("type"), Value: []byte("Uploaded")}
	headers := []sarama.RecordHeader{typeHeader}
	msg := &sarama.ProducerMessage{Topic: "videos", Key: sarama.StringEncoder(id), Value: sarama.ByteEncoder(m), Headers: headers}

	_, _, err = sp.SendMessage(msg)
	if err != nil {
		return err
	}
	return nil
}
