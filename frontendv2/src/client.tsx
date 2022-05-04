import axios from 'axios';
import { Video } from 'tabler-icons-react';
import {
  GrpcWebFetchTransport,
  GrpcWebOptions,
} from '@protobuf-ts/grpcweb-transport';
import { VideoServiceClient } from './gen/videos/v1/videos.client';
import { CreateVideoRequest } from './gen/videos/v1/videos';
import { SearchServiceClient } from './gen/search/v1/search.client';
import { SearchVideosRequest } from './gen/search/v1/search';

const BASE = import.meta.env.VITE_API_URL || 'http://api.msostream.io/';

const client = axios.create({
  withCredentials: true,
  baseURL: BASE,
});

const storedJWT = localStorage.getItem('msostream-user');

if (storedJWT) {
  client.defaults.headers.common['authorization'] = storedJWT;
}

const transport = () => {
  const jwt = localStorage.getItem('msostream-user');
  const conf: GrpcWebOptions = { baseUrl: BASE };
  if (jwt != null) {
    conf.meta = {
      authorization: jwt,
    };
  }

  return new GrpcWebFetchTransport(conf);
};

export interface Video {
  id: string;
  title: string;
  description: string;
  author: string;
}

const getVideos = (callback: (arg0: Video[]) => unknown) => {
  const req: SearchVideosRequest = {
    query: '',
    offset: BigInt(0),
    limit: BigInt(10),
    filter: [],
    sort: [],
  };

  return client
    .get('/v1/search/videos', {
      data: JSON.stringify({ query: '' }),
      withCredentials: false,
    })
    .then((resp) => callback(resp.data.videos));
};

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
  const client = new VideoServiceClient(transport());
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
export { getVideos, login, register, createVideo, uploadVideo, transport };
