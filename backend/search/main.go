package main

import (
	"msostream/search/collectors"
	"os"
	"os/signal"
	"sync"

	"github.com/meilisearch/meilisearch-go"
)

var CLIENT meilisearch.Client

func main() {

	client := InitMeilisearch()
	initResult := collectors.InitCollectors(client)

	var wg sync.WaitGroup
	wg.Add(len(initResult.Topics))
	collectors.CollectTopics(&wg, initResult.Topics)
	wg.Wait()
	InitGrpc()

	signals := make(chan os.Signal, 1)
	signal.Notify(signals, os.Interrupt)
	<-signals
}
