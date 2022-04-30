import axios from 'axios';
import Cookies from 'js-cookie';
import { Video } from 'tabler-icons-react';

const BASE = import.meta.env.VITE_API_URL || 'https://msostream.io/';

const client = axios.create({
  withCredentials: true,
  baseURL: BASE,
});

const cookie = Cookies.get('msostream-user');

if (cookie) {
  client.defaults.headers.common['authorization'] = cookie;
}

export interface Video {
  id: string;
  title: string;
  description: string;
  user_id: string;
}

const getVideos = (callback: (arg0: Video[]) => any) =>
  client.get('video/list').then((resp) => callback(resp.data));

const setJWT = (jwt: string) => {
    localStorage.setItem('msostream-user', jwt);
    client.defaults.headers.common['authorization'] = jwt;
    return jwt;
}

const login = (email: string, password: string) =>
  client.post('/v1/users/login', { email, password }).then((resp) => {
    return setJWT(resp.headers['authorization']);
  });

const register = (name: string, email: string, password: string) =>
  client.post('/v1/users/register', { name, email, password}).then((resp) => {
    return setJWT(resp.headers['authorization']);
  })

export default client;
export { getVideos, login, register };
