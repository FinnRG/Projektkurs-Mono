import axios from 'axios';
import { wrapper } from 'axios-cookiejar-support';
import { CookieJar } from 'tough-cookie';

const process = require('process');

const jar = new CookieJar();
const client = wrapper(axios.create({
    jar,
    withCredentials: true,
    baseURL: process.env.GITPOD_WORKSPACE_URL || 'http://localhost:8000'
}));
export default client;