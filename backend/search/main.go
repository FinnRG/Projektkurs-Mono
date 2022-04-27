package main

import (
	"os"
	"os/signal"
	"sync"

	"github.com/meilisearch/meilisearch-go"
)

var CLIENT meilisearch.Client
var TOPICS []string = []string{"videos"}

func main() {

	initEnv()
	InitKafka()

	var wg sync.WaitGroup
	wg.Add(len(TOPICS))
	CollectTopics(&wg, TOPICS)
	wg.Wait()
	initGrpc()

	signals := make(chan os.Signal, 1)
	signal.Notify(signals, os.Interrupt)
	<-signals
}

func initEnv() {
	host := os.Getenv("MEILISEARCH_URL")
	if len(host) == 0 {
		host = "http://meilisearch:7700"
	}

	CLIENT = *meilisearch.NewClient(meilisearch.ClientConfig{
		Host: host,
	})
}
