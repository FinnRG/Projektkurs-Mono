package main

import (
	"context"
	"fmt"
	"log"
	"net"
	"reflect"

	searchv1 "msostream/search/gen/search/v1"
	v1 "msostream/search/gen/search/v1"

	"github.com/meilisearch/meilisearch-go"
	"github.com/mitchellh/mapstructure"
	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection"
)

func initGrpc() {
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
		Filter: req.Filter,
		Sort:   req.Sort,
	})
	if err != nil {
		log.Printf("Meilisearch error: %s\n", err)
		return &searchv1.SearchVideosResponse{}, err
	}

	videos := make([]*searchv1.Video, len(res.Hits))
	for i, arg := range res.Hits {

		videos[i] = decodeVideo(arg)
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

func decodeVideo(arg interface{}) *searchv1.Video {
	var video searchv1.Video

	decodeHook := func(from reflect.Type, to reflect.Type, v interface{}) (interface{}, error) {
		var vis v1.Visibility
		if to == reflect.TypeOf(vis) {
			switch v.(string) {
			case "DRAFT":
				return v1.Visibility_VISIBILITY_DRAFT, nil
			case "PROCESSING":
				return v1.Visibility_VISIBILITY_PROCESSING, nil
			case "PRIVATE":
				return v1.Visibility_VISIBILITY_PRIVATE, nil
			case "PUBLIC":
				return v1.Visibility_VISIBILITY_PUBLIC, nil
			default:
				return v1.Visibility_VISIBILITY_UNSPECIFIED, nil
			}
		}
		return v, nil
	}

	config := &mapstructure.DecoderConfig{
		DecodeHook: decodeHook,
		Result:     &video,
	}

	decoder, err := mapstructure.NewDecoder(config)
	if err != nil {
		log.Fatalf("decoder config error: %v\n", err)
	}

	err = decoder.Decode(arg)
	if err != nil {
		log.Fatalf("decoding error: %s", err)
	}
	return &video
}
