package main

import (
	"encoding/json"
	"log"
	"os"
	"os/signal"
	"sync"

	"github.com/Shopify/sarama"
	"github.com/meilisearch/meilisearch-go"
)

type Video struct {
	Id          string `json:"id"`
	Title       string `json:"title"`
	Description string `json:"description"`
	Author      string `json:"author"`
	Date        string `json:"date"`
	Visbility   string `json:"visibility"`
}

var KAFKA_URL = os.Getenv("KAFKA_URL")

func InitKafka() {
	if len(KAFKA_URL) == 0 {
		KAFKA_URL = "kafka:9092"
		log.Printf("KAFKA_URL set to default: %s\n", KAFKA_URL)
	}
}

func CollectTopics(wg *sync.WaitGroup, topics []string) {
	log.Printf("Started collecting videos from: %s", topics)
	config := sarama.NewConfig()
	consumer, err := sarama.NewConsumer([]string{KAFKA_URL}, config)
	if err != nil {
		log.Fatalln(err)
	}

	signals := make(chan os.Signal, 1)
	signal.Notify(signals, os.Interrupt)

	log.Println("Creating PartitionConsumer")
	for _, t := range topics {

		go CollectTopic(consumer, t, signals, wg)

	}
}

func getLastOffset(topic string) (int64, error) {
	client, err := sarama.NewClient([]string{KAFKA_URL}, sarama.NewConfig())
	if err != nil {
		return -1, err
	}

	return client.GetOffset(topic, 0, sarama.OffsetNewest)

}

func CollectTopic(consumer sarama.Consumer, topic string, signals chan os.Signal, wg *sync.WaitGroup) error {

	_, err := CLIENT.CreateIndex(&meilisearch.IndexConfig{
		Uid:        topic,
		PrimaryKey: "id",
	})
	if err != nil {
		log.Println(err)
	}

	lOff, err := getLastOffset(topic)
	if err != nil {
		log.Fatalln(err)
	}

	// This is triggered if the topic is empty
	if lOff == 0 {
		wg.Done()
	}
	partitionConsumer, err := consumer.ConsumePartition(topic, 0, sarama.OffsetOldest)
	if err != nil {
		log.Fatalln(err)
	}

	defer func() {
		if err := partitionConsumer.Close(); err != nil {
			log.Fatalln(err)
		}
	}()

	index := CLIENT.Index(topic)

CollectLoop:
	for {
		select {
		case msg := <-partitionConsumer.Messages():
			processMessage(msg, index)
			if msg.Offset == lOff-1 && lOff != 0 {
				wg.Done()
			}
		case <-signals:
			break CollectLoop
		}
	}

	return nil
}

func processMessage(msg *sarama.ConsumerMessage, index *meilisearch.Index) {
	t, err := getMessageType(msg)
	// Not necessarily an error
	if err != nil {
		return
	}

	var obj map[string]interface{}
	err = json.Unmarshal(msg.Value, &obj)
	if err != nil {
		log.Fatalln(err)
	}

	switch t {
	case "Updated", "Processed":
		index.AddDocuments([]map[string]interface{}{obj})
	case "Deleted":
		index.DeleteDocument(obj["id"].(string))
	}

}

func getMessageType(msg *sarama.ConsumerMessage) (string, error) {
	for _, h := range msg.Headers {
		if string(h.Key) == "type" {
			return string(h.Value), nil
		}
	}
	return "", nil
}
