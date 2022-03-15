import axios from 'axios';
import { wrapper } from 'axios-cookiejar-support';
import { CookieJar } from 'tough-cookie';

const jar = new CookieJar();

let pod_url = process.env.REACT_APP_API_URl;
if (!pod_url) {
  pod_url = 'http://localhost:8000';
}

const client = wrapper(
  axios.create({
    jar,
    withCredentials: true,
    baseURL: pod_url,
  })
);
export default client;
export { pod_url };
