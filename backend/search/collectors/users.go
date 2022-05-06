package collectors

import (
	"log"
	"os"
	"sync"

	"github.com/Shopify/sarama"
	"github.com/meilisearch/meilisearch-go"
	"github.com/tidwall/gjson"
)

func CollectUsers(consumer sarama.Consumer, topic string, signals chan os.Signal, wg *sync.WaitGroup) error {

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

	if lOff == 0 {
		wg.Done()
	}
	partitionConsumer, err := consumer.ConsumePartition(topic, 0, sarama.OffsetOldest)
	if err != nil {
		log.Fatalln(err)
	}

	defer func() {
		if err := partitionConsumer.Close(); err != nil {
			log.Default().Fatalln(err)
		}
	}()

	index := CLIENT.Index(topic)
CollectLoop:
	for {
		select {
		case msg := <-partitionConsumer.Messages():
			processUserEvent(msg, index, topic)
			if msg.Offset == lOff-1 && lOff != 0 {
				wg.Done()
			}
		case <-signals:
			break CollectLoop
		}
	}
	return nil
}

func processUserEvent(msg *sarama.ConsumerMessage, index *meilisearch.Index, topic string) {
	t, err := getMessageType(msg)
	if err != nil {
		return
	}

	switch t {
	case "Registered", "Changed":
		updateDocumentString(index, msg.Key)
	case "NameChanged":
		// TODO: Implement this
		// videos := getVideosForUser(topic, string(msg.Key), msg.Offset)
	case "Deleted":
		id := gjson.Get(string(msg.Value), "id")
		index.DeleteDocument(id.String())
	}
}

func getVideosForUser(videoTopic string, userId string, offset int64) []string {
	config := sarama.NewConfig()
	consumer, err := sarama.NewConsumer([]string{KAFKA_URL}, config)
	if err != nil {
		log.Fatalln(err)
	}
	partitionConsumer, err := consumer.ConsumePartition(videoTopic, 0, sarama.OffsetOldest)
	if err != nil {
		log.Fatalln(err)
	}
	defer func() {
		if err := partitionConsumer.Close(); err != nil {
			log.Fatalln(err)
		}
	}()

	videos := []string{}

	for {
		msg := <-partitionConsumer.Messages()
		t, err := getMessageType(msg)

		if err != nil {
			log.Fatalln(err)
		}

		if t == "Finished" {
			author := gjson.Get(string(msg.Value), "title")
			if author.Exists() && author.String() == userId {
				videos = append(videos, string(msg.Key))
			}
		}

		if msg.Offset == offset {
			break
		}
	}

	return videos
}
