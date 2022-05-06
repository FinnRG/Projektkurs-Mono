package collectors

import (
	"log"
	"os"
	"sync"

	"github.com/Shopify/sarama"
	"github.com/meilisearch/meilisearch-go"
)

func CollectVideos(consumer sarama.Consumer, topic string, signals chan os.Signal, wg *sync.WaitGroup) error {

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
			processVideoEvent(msg, index)
			if msg.Offset == lOff-1 && lOff != 0 {
				wg.Done()
			}
		case <-signals:
			break CollectLoop
		}
	}

	return nil
}

// TODO: Grab the video with the id to check for visibility, etc.
func processVideoEvent(msg *sarama.ConsumerMessage, index *meilisearch.Index) {
	t, err := getMessageType(msg)
	// Not necessarily a critical error
	if err != nil {
		return
	}

	switch t {
	case "TitleChanged", "DescriptionChanged", "VisibilityChanged", "Finished":
		updateDocumentString(index, msg.Value)
	case "Deleted":
		index.DeleteDocument(string(msg.Key))
	}

}
