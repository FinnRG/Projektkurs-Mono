import axios from 'axios';
import Cookies from 'js-cookie';
import { Video } from 'tabler-icons-react';

const client = axios.create({
  withCredentials: true,
  baseURL: import.meta.env.VITE_API_URL || 'http://localhost:8000/',
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
  client.post('login', { email, password }).then((resp) => {
    return setJWT(resp.headers['authorization']);
  });

const register = (email: string, password: string) =>
  client.post('register', { name, email, password}).then((resp) => {
    return setJWT(resp.headers['authorization']);
  })

export default client;
export { getVideos, login, register };
