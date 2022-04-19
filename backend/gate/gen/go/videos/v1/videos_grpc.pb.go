// Code generated by protoc-gen-go-grpc. DO NOT EDIT.

package gate

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// VideoServiceClient is the client API for VideoService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type VideoServiceClient interface {
	CreateVideo(ctx context.Context, in *CreateVideoRequest, opts ...grpc.CallOption) (*CreateVideoResponse, error)
	UpdateVideo(ctx context.Context, in *UpdateVideoRequest, opts ...grpc.CallOption) (*UpdateVideoResponse, error)
	GetVideo(ctx context.Context, in *GetVideoRequest, opts ...grpc.CallOption) (*GetVideoResponse, error)
}

type videoServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewVideoServiceClient(cc grpc.ClientConnInterface) VideoServiceClient {
	return &videoServiceClient{cc}
}

func (c *videoServiceClient) CreateVideo(ctx context.Context, in *CreateVideoRequest, opts ...grpc.CallOption) (*CreateVideoResponse, error) {
	out := new(CreateVideoResponse)
	err := c.cc.Invoke(ctx, "/videos.v1.VideoService/CreateVideo", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *videoServiceClient) UpdateVideo(ctx context.Context, in *UpdateVideoRequest, opts ...grpc.CallOption) (*UpdateVideoResponse, error) {
	out := new(UpdateVideoResponse)
	err := c.cc.Invoke(ctx, "/videos.v1.VideoService/UpdateVideo", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *videoServiceClient) GetVideo(ctx context.Context, in *GetVideoRequest, opts ...grpc.CallOption) (*GetVideoResponse, error) {
	out := new(GetVideoResponse)
	err := c.cc.Invoke(ctx, "/videos.v1.VideoService/GetVideo", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// VideoServiceServer is the server API for VideoService service.
// All implementations must embed UnimplementedVideoServiceServer
// for forward compatibility
type VideoServiceServer interface {
	CreateVideo(context.Context, *CreateVideoRequest) (*CreateVideoResponse, error)
	UpdateVideo(context.Context, *UpdateVideoRequest) (*UpdateVideoResponse, error)
	GetVideo(context.Context, *GetVideoRequest) (*GetVideoResponse, error)
	mustEmbedUnimplementedVideoServiceServer()
}

// UnimplementedVideoServiceServer must be embedded to have forward compatible implementations.
type UnimplementedVideoServiceServer struct {
}

func (UnimplementedVideoServiceServer) CreateVideo(context.Context, *CreateVideoRequest) (*CreateVideoResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreateVideo not implemented")
}
func (UnimplementedVideoServiceServer) UpdateVideo(context.Context, *UpdateVideoRequest) (*UpdateVideoResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UpdateVideo not implemented")
}
func (UnimplementedVideoServiceServer) GetVideo(context.Context, *GetVideoRequest) (*GetVideoResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetVideo not implemented")
}
func (UnimplementedVideoServiceServer) mustEmbedUnimplementedVideoServiceServer() {}

// UnsafeVideoServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to VideoServiceServer will
// result in compilation errors.
type UnsafeVideoServiceServer interface {
	mustEmbedUnimplementedVideoServiceServer()
}

func RegisterVideoServiceServer(s grpc.ServiceRegistrar, srv VideoServiceServer) {
	s.RegisterService(&VideoService_ServiceDesc, srv)
}

func _VideoService_CreateVideo_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CreateVideoRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(VideoServiceServer).CreateVideo(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/videos.v1.VideoService/CreateVideo",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(VideoServiceServer).CreateVideo(ctx, req.(*CreateVideoRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _VideoService_UpdateVideo_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UpdateVideoRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(VideoServiceServer).UpdateVideo(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/videos.v1.VideoService/UpdateVideo",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(VideoServiceServer).UpdateVideo(ctx, req.(*UpdateVideoRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _VideoService_GetVideo_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetVideoRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(VideoServiceServer).GetVideo(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/videos.v1.VideoService/GetVideo",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(VideoServiceServer).GetVideo(ctx, req.(*GetVideoRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// VideoService_ServiceDesc is the grpc.ServiceDesc for VideoService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var VideoService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "videos.v1.VideoService",
	HandlerType: (*VideoServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "CreateVideo",
			Handler:    _VideoService_CreateVideo_Handler,
		},
		{
			MethodName: "UpdateVideo",
			Handler:    _VideoService_UpdateVideo_Handler,
		},
		{
			MethodName: "GetVideo",
			Handler:    _VideoService_GetVideo_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "videos/v1/videos.proto",
}
