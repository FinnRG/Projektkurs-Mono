package collectors

import (
	"encoding/json"
	"log"
	searchv1 "msostream/search/gen/search/v1"

	"github.com/Shopify/sarama"
	"github.com/meilisearch/meilisearch-go"
)

func getLastOffset(topic string) (int64, error) {
	client, err := sarama.NewClient([]string{KAFKA_URL}, sarama.NewConfig())
	if err != nil {
		return -1, err
	}

	return client.GetOffset(topic, 0, sarama.OffsetNewest)

}

// Returns the value of the type header, if it exists
func getMessageType(msg *sarama.ConsumerMessage) (string, error) {
	for _, h := range msg.Headers {
		if string(h.Key) == "type" {
			return string(h.Value), nil
		}
	}
	return "", nil
}

func unsafeDeserialize(str []byte) map[string]interface{} {
	var obj map[string]interface{}
	err := json.Unmarshal(str, &obj)
	if err != nil {
		log.Fatalln(err)
	}
	return obj
}

func updateDocumentString(index *meilisearch.Index, str []byte) {
	obj := unsafeDeserialize(str)
	index.AddDocuments([]map[string]interface{}{obj})
}

func createExtendedVideo(video searchv1.Video, authorName string) *searchv1.ExtendedVideo {
	return &searchv1.ExtendedVideo{
		Id:          video.Id,
		Title:       video.Title,
		Description: video.Description,
		Date:        video.Date,
		Visibility:  video.Visibility,
		Status:      video.Status,
		AuthorId:    video.Author,
		AuthorName:  authorName,
	}
}
