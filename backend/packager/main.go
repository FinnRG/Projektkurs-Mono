package main

import (
	"errors"
	"log"
	"msostream/packager/kafka"
	"msostream/packager/minio"
	"msostream/packager/tasks"
	"os"
	"os/signal"
	"sync"

	"github.com/hibiken/asynq"
)

var REDIS_URL string = os.Getenv("REDIS_URL")
var REDISPASSWORD string = os.Getenv("REDISPASSWORD")

func main() {
	initEnv()
	go runAsynqClient()
	runAsynqWorker()
}

func runAsynqClient() {
	client := asynq.NewClient(asynq.RedisClientOpt{Addr: REDIS_URL, Password: REDISPASSWORD, DB: 0})
	defer client.Close()

	signals := make(chan os.Signal, 1)
	signal.Notify(signals, os.Interrupt)

SearchLoop:
	for {
		select {
		case msg := <-kafka.WorkChan:
			if kafka.IsProcessed(msg.Id) {
				log.Printf("%v was already processed, skipping", msg.Id)
			} else {
				task, err := tasks.NewVideoTranscodeTask(msg)
				if err != nil {
					log.Fatalf("Could not create task: %v", err)
				}
				info, err := client.Enqueue(task, asynq.TaskID(msg.Id))
				if err != nil && !errors.Is(err, asynq.ErrTaskIDConflict) {
					log.Fatalf("Could not enqueue task: %v", err)
				} else if err != nil {
					log.Printf("Duplicate task")
				} else {
					log.Printf("Enqueued task: id=%s queue=%s", info.ID, info.Queue)
				}
			}
		case <-signals:
			break SearchLoop
		}
	}

}

func initEnv() {
	if len(REDIS_URL) == 0 {
		REDIS_URL = "redis:6379"
		log.Printf("Using default REDIS_URL: %s", REDIS_URL)
	}
	if len(REDISPASSWORD) == 0 {
		log.Fatalf("No redis password specified")
	}

	kafka.InitKafka()

	var wg sync.WaitGroup
	wg.Add(2)
	go minio.InitMinio(&wg)
	go kafka.CollectVideos(&wg)
	wg.Wait()
}

func runAsynqWorker() {
	log.Println("Started asynq worker")
	srv := asynq.NewServer(
		asynq.RedisClientOpt{Addr: REDIS_URL, Password: REDISPASSWORD, DB: 0},
		asynq.Config{
			Concurrency: 3,
		},
	)

	mux := asynq.NewServeMux()
	mux.HandleFunc(tasks.TypeVideoTranscode, tasks.HandleVideoTranscodeTask)

	if err := srv.Run(mux); err != nil {
		log.Fatalf("Could not run server: %v", err)
	}
}
