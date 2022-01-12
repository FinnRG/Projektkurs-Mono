import axios from 'axios';
import { wrapper } from 'axios-cookiejar-support';
import { CookieJar } from 'tough-cookie';

const jar = new CookieJar();

let pod_url = process.env.REACT_APP_WORKSPACE_URL || 'http://localhost:8000';
if (pod_url) {
    pod_url = [pod_url.slice(0, 8), '8000-', pod_url.slice(8)].join('');
}

const client = wrapper(axios.create({
    jar,
    withCredentials: true,
    baseURL: pod_url
}));
export default client;
export { pod_url };