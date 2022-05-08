package collectors

import (
	"log"
	"os"
	"sync"

	"github.com/Shopify/sarama"
	"github.com/golang/protobuf/proto"
	"github.com/meilisearch/meilisearch-go"

	searchv1 "msostream/search/gen/search/v1"
	usersv1 "msostream/search/gen/users/v1"
	videosv1 "msostream/search/gen/videos/v1"
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
	case "Finished":
		processVideoFinished(msg, index)
	case "Created":
		processVideoCreated(msg, index)
	case "TitleChanged", "DescriptionChanged", "VisibilityChanged":
		updateDocumentString(index, msg.Value)
	case "Deleted":
		index.DeleteDocument(string(msg.Key))
	}

}

func processVideoFinished(msg *sarama.ConsumerMessage, index *meilisearch.Index) {
	var video searchv1.ExtendedVideo
	err := index.GetDocument(string(msg.Key), &video)
	if err != nil {
		log.Fatalln(err)
	}

	userIndex := CLIENT.Index("users")

	var user usersv1.PublicUserInfo
	userIndex.GetDocument(video.AuthorId, user)

	index.UpdateDocuments([]searchv1.ExtendedVideo{{
		Status: videosv1.Status_STATUS_FINISHED,
		Id:     video.Id,
	}})
}

func processVideoCreated(msg *sarama.ConsumerMessage, index *meilisearch.Index) {
	var event videosv1.VideoCreatedEvent
	err := proto.Unmarshal(msg.Value, &event)
	if err != nil {
		log.Fatalln(err)
	}
	log.Println("Title: %v", event.Title)
	log.Println("Title: %v", event.Id)
	log.Println("Title: %v", event.Visibility)

	userIndex := CLIENT.Index("users")

	var user usersv1.PublicUserInfo
	userIndex.GetDocument(event.Author, &user)

	index.AddDocuments([]searchv1.ExtendedVideo{{
		Id:          event.Id,
		Title:       event.Title,
		Description: event.Description,
		Date:        event.Date,
		Visibility:  event.Visibility,
		Status:      searchv1.Status_STATUS_DRAFT,
		AuthorId:    event.Author,
		AuthorName:  user.Name,
	}})
}
