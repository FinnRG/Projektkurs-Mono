package tasks

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"msostream/packager/kafka"
	"msostream/packager/transcode"

	"github.com/hibiken/asynq"
)

const (
	TypeVideoTranscode = "video:transcode"
)

type VideoTranscodePayload = kafka.Video

func NewVideoTranscodeTask(video VideoTranscodePayload) (*asynq.Task, error) {
	payload, err := json.Marshal(video)
	if err != nil {
		return nil, err
	}
	return asynq.NewTask(TypeVideoTranscode, payload), nil
}

func HandleVideoTranscodeTask(ctx context.Context, t *asynq.Task) error {
	log.Println("Starting to handle new VideoTranscodeTask")
	var p VideoTranscodePayload
	if err := json.Unmarshal(t.Payload(), &p); err != nil {
		return fmt.Errorf("json.Unmarshal failed: %v: %w", err, asynq.SkipRetry)
	}
	return transcode.TranscodeAndUpload(p.Id)
}
