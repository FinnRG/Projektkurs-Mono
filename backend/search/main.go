package main

import (
	"msostream/search/collectors"
	"os"
	"os/signal"

	"github.com/meilisearch/meilisearch-go"
)

var CLIENT meilisearch.Client

func main() {

	client := InitMeilisearch()
	initResult := collectors.InitCollectors(client)

	collectors.CollectTopics(initResult.Topics)
	InitGrpc()

	signals := make(chan os.Signal, 1)
	signal.Notify(signals, os.Interrupt)
	<-signals
}
