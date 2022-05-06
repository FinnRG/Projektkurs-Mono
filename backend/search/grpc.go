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
		Filter: req.Filter,
		Sort:   req.Sort,
	})
	if err != nil {
		log.Printf("Meilisearch error: %s\n", err)
		return &searchv1.SearchVideosResponse{}, err
	}

	videos := make([]*searchv1.ExtendedVideo, len(res.Hits))
	for i, arg := range res.Hits {

		videos[i] = &searchv1.ExtendedVideo{
			Video:  decodeVideo(arg),
			Author: &searchv1.PublicUserInfo{},
		}
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
			case "VISIBILITY_PRIVATE":
				return v1.Visibility_VISIBILITY_PRIVATE, nil
			case "VISIBILITY_PUBLIC":
				return v1.Visibility_VISIBILITY_PUBLIC, nil
			default:
				return v1.Visibility_VISIBILITY_UNSPECIFIED, nil
			}
		}
		var status v1.Status
		if to == reflect.TypeOf(status) {
			switch v.(string) {
			case "STATUS_DRAFT":
				return v1.Status_STATUS_DRAFT, nil
			case "STATUS_UPLOADED":
				return v1.Status_STATUS_UPLOADED, nil
			case "STATUS_PROCESSED":
				return v1.Status_STATUS_PROCESSED, nil
			case "STATUS_FINISHED":
				return v1.Status_STATUS_FINISHED, nil
			default:
				return v1.Status_STATUS_UNSPECIFIED, nil
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
