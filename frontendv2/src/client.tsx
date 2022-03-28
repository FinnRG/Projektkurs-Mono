import axios from 'axios';
import { Video } from 'tabler-icons-react';

const client = axios.create({
  withCredentials: true,
  baseURL: import.meta.env.VITE_API_URL || 'http://localhost:8000/',
});

export interface Video {
  id: String;
  title: string;
  description: string;
  user_id: string;
}
const getVideos = (callback: (arg0: Video[]) => any) =>
  client.get('video/list').then((resp) => callback(resp.data));

export default client;
export { getVideos };
