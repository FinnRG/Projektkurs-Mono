package collectors

import (
	"log"
	"os"
	"os/signal"
	"sync"

	"github.com/Shopify/sarama"
	"github.com/meilisearch/meilisearch-go"
)

var KAFKA_URL = os.Getenv("KAFKA_URL")
var CLIENT *meilisearch.Client
var TOPICS []TopicCallbackTuple = []TopicCallbackTuple{{
	topic:    "users",
	callback: CollectUsers,
}, {
	topic:    "videos",
	callback: CollectVideos,
}}

type TopicCallbackTuple struct {
	topic    string
	callback func(sarama.Consumer, string, chan os.Signal, *sync.WaitGroup) error
}

type InitResult struct {
	Kafka_URL string
	Topics    []TopicCallbackTuple
}

func InitCollectors(client *meilisearch.Client) InitResult {
	if len(KAFKA_URL) == 0 {
		KAFKA_URL = "kafka:9092"
		log.Printf("KAFKA_URL set to default: %s\n", KAFKA_URL)
	}
	CLIENT = client

	return InitResult{
		Kafka_URL: KAFKA_URL,
		Topics:    TOPICS,
	}
}

func CollectTopics(topics []TopicCallbackTuple) {

	config := sarama.NewConfig()
	consumer, err := sarama.NewConsumer([]string{KAFKA_URL}, config)
	if err != nil {
		log.Fatalln(err)
	}

	signals := make(chan os.Signal, 1)
	signal.Notify(signals, os.Interrupt)

	log.Println("Creating PartitionConsumer")
	for _, t := range topics {
		var wg sync.WaitGroup
		wg.Add(1)
		log.Printf("Starting consumer for %v", t)
		go t.callback(consumer, t.topic, signals, &wg)
		wg.Wait()
	}
}
