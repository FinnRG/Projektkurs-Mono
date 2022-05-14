package tasks

import (
	"context"
	"fmt"
	"log"
	videosv1 "msostream/packager/gen/go/videos/v1"
	"msostream/packager/transcode"

	"github.com/hibiken/asynq"
	"google.golang.org/protobuf/proto"
)

const (
	TypeVideoTranscode = "video:transcode"
)

type VideoTranscodePayload = videosv1.VideoUploadedEvent

func NewVideoTranscodeTask(video *VideoTranscodePayload) (*asynq.Task, error) {
	payload, err := proto.Marshal(video)
	if err != nil {
		return nil, err
	}
	return asynq.NewTask(TypeVideoTranscode, payload), nil
}

func HandleVideoTranscodeTask(ctx context.Context, t *asynq.Task) error {
	log.Println("Starting to handle new VideoTranscodeTask")
	var p VideoTranscodePayload
	if err := proto.Unmarshal(t.Payload(), &p); err != nil {
		return fmt.Errorf("proto.Unmarshal failed: %v: %w", err, asynq.SkipRetry)
	}
	return transcode.TranscodeAndUpload(p.Id)
}
