import axios from 'axios';
import { Video } from 'tabler-icons-react';
import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport';
import { VideoServiceClient } from './gen/videos/v1/videos.client';
import { CreateVideoRequest } from './gen/videos/v1/videos';

const BASE = import.meta.env.VITE_API_URL || 'https://msostream.io/';

const client = axios.create({
  withCredentials: true,
  baseURL: BASE,
});

const storedJWT = localStorage.getItem('msostream-user');

if (storedJWT) {
  client.defaults.headers.common['authorization'] = storedJWT;
}

export interface Video {
  id: string;
  title: string;
  description: string;
  user_id: string;
}

const getVideos = (callback: (arg0: Video[]) => unknown) =>
  client.get('/v1/search/videos').then((resp) => callback(resp.data));

const setJWT = (jwt: string) => {
  localStorage.setItem('msostream-user', jwt);
  client.defaults.headers.common['authorization'] = jwt;
  return jwt;
};

const login = (email: string, password: string) =>
  client.post('/v1/users/login', { email, password }).then((resp) => {
    return setJWT(resp.headers['authorization']);
  });

const register = (name: string, email: string, password: string) =>
  client.post('/v1/users/register', { name, email, password }).then((resp) => {
    return setJWT(resp.headers['authorization']);
  });

const createVideo = (req: CreateVideoRequest) => {
  const jwt = localStorage.getItem('msostream-user');
  if (jwt == null) {
    return;
  }

  const transport = new GrpcWebFetchTransport({
    baseUrl: BASE,
    meta: {
      authorization: jwt,
    },
  });

  const client = new VideoServiceClient(transport);
  return client.createVideo(req);
};

const uploadVideo = (id: string, file: File) => {
  const formData = new FormData();
  formData.append('file', file);
  return client.post(`/upload/${id}`, formData, {
    headers: {
      'Content-Type': 'multipart/form-data',
    },
  });
};

export default client;
export { getVideos, login, register, createVideo, uploadVideo };
