package collectors

import (
	"log"
	"os"
	"sync"

	"github.com/Shopify/sarama"
	"github.com/golang/protobuf/proto"
	"github.com/meilisearch/meilisearch-go"
	"github.com/tidwall/gjson"

	searchv1 "msostream/search/gen/search/v1"
	usersv1 "msostream/search/gen/users/v1"
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
	log.Println("New user event of type %v", t)

	switch t {
	case "Registered":
		processUserRegistered(msg, index)
	case "NameChanged":
		processUserNameChanged(msg, index)
	case "Deleted":
		id := gjson.Get(string(msg.Value), "id")
		index.DeleteDocument(id.String())
	}
}

func processUserRegistered(msg *sarama.ConsumerMessage, index *meilisearch.Index) {
	var event usersv1.UserRegisteredEvent
	err := proto.Unmarshal(msg.Value, &event)
	if err != nil {
		log.Fatalln(err)
	}

	index.AddDocuments([]usersv1.UserRegisteredEvent{event})
}

func processUserNameChanged(msg *sarama.ConsumerMessage, index *meilisearch.Index) {
	videos := getVideosForUser(string(msg.Key), msg.Offset)
	var event usersv1.UserNameChangedEvent
	err := proto.Unmarshal(msg.Value, &event)
	if err != nil {
		log.Fatalln(err)
	}

	var res []searchv1.ExtendedVideo

	for i, videoId := range videos {
		res[i] = searchv1.ExtendedVideo{
			Id:         videoId,
			AuthorName: event.Name,
		}
	}

	index.UpdateDocuments(res)
}

// TODO: Don't use json
func getVideosForUser(userId string, offset int64) []string {
	config := sarama.NewConfig()
	consumer, err := sarama.NewConsumer([]string{KAFKA_URL}, config)
	if err != nil {
		log.Fatalln(err)
	}
	partitionConsumer, err := consumer.ConsumePartition("videos", 0, sarama.OffsetOldest)
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
			author := gjson.Get(string(msg.Value), "author")
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
