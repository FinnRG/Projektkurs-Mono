package collectors

import "github.com/Shopify/sarama"

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
