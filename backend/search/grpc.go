package main

import (
	"context"
	"fmt"
	"log"
	"net"
	"regexp"
	"strings"

	searchv1 "msostream/search/gen/search/v1"

	"github.com/meilisearch/meilisearch-go"
	"github.com/mitchellh/mapstructure"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"
	"google.golang.org/grpc/reflection"
)

func InitGrpc() {
	if err := runGrpc(); err != nil {
		log.Fatal(err)
	}
}

func runGrpc() error {
	listenOn := "0.0.0.0:8080"
	listener, err := net.Listen("tcp", listenOn)
	if err != nil {
		return fmt.Errorf("failed to listen on %s: %w", listenOn, err)
	}

	server := grpc.NewServer()
	searchv1.RegisterSearchServiceServer(server, &searchServiceServer{})
	reflection.Register(server)
	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	log.Println("Listening on", listenOn)
	if err := server.Serve(listener); err != nil {
		return fmt.Errorf("failed to serve gRPC server: %w", err)
	}

	return nil
}

type searchServiceServer struct {
	searchv1.UnimplementedSearchServiceServer
}

func (s *searchServiceServer) SearchVideos(ctx context.Context, req *searchv1.SearchVideosRequest) (*searchv1.SearchVideosResponse, error) {
	log.Println("New query")
	res, err := CLIENT.Index("videos").Search(req.Query, &meilisearch.SearchRequest{
		Offset: int64(req.Offset),
		Limit:  int64(req.Limit),
		Filter: append(req.Filter, ""),
		Sort:   req.Sort,
	})
	if err != nil {
		log.Printf("Meilisearch error: %s\n", err)
		return &searchv1.SearchVideosResponse{}, err
	}

	videos := make([]*searchv1.ExtendedVideo, len(res.Hits))
	for i, arg := range res.Hits {
		log.Printf("Video: %v", arg)
		var video *searchv1.ExtendedVideo
		config := &mapstructure.DecoderConfig{
			MatchName: matchName,
			Result:    &video,
		}

		decoder, err := mapstructure.NewDecoder(config)
		if err != nil {
			log.Fatalln(err)
		}
		decoder.Decode(arg)

		videos[i] = video
	}

	log.Println("Successful query")
	return &searchv1.SearchVideosResponse{
		Videos:           videos,
		Offset:           uint64(res.Offset),
		Limit:            uint64(res.Limit),
		NbHits:           uint64(res.NbHits),
		ExhaustiveNbHits: res.ExhaustiveNbHits,
	}, nil
}

func matchName(mapKey, fieldName string) bool {
	return strings.EqualFold(ToSnakeCase(mapKey), ToSnakeCase(fieldName))
}

// Source: https://stackoverflow.com/questions/56616196/how-to-convert-camel-case-string-to-snake-case
var matchFirstCap = regexp.MustCompile("(.)([A-Z][a-z]+)")
var matchAllCap = regexp.MustCompile("([a-z0-9])([A-Z])")

func ToSnakeCase(str string) string {
	snake := matchFirstCap.ReplaceAllString(str, "${1}_${2}")
	snake = matchAllCap.ReplaceAllString(snake, "${1}_${2}")
	return strings.ToLower(snake)
}
