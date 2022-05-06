package main

import (
	"os"

	"github.com/meilisearch/meilisearch-go"
)

func InitMeilisearch() *meilisearch.Client {
	host := os.Getenv("MEILISEARCH_URL")
	if len(host) == 0 {
		host = "http://meilisearch:7700"
	}

	CLIENT = *meilisearch.NewClient(meilisearch.ClientConfig{
		Host: host,
	})

	return &CLIENT
}
