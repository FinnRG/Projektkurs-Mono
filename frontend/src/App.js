import axios from 'axios';
import { wrapper } from 'axios-cookiejar-support';
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { CookieJar } from 'tough-cookie';
import Player from './components/player/Player';
import Upload from './components/upload/Upload';
import { Layout } from './Layout';
import Id from './pages/Id';
import Login from './pages/Login';
import Register from './pages/Register';

const jar = new CookieJar();
export const client = wrapper(axios.create({ jar }));

export const App = () => {

    return <BrowserRouter>
        <Routes>
            <Route path='/' element={<Layout />}>
                <Route path='upload' element={<Upload />} />
                <Route path='player' element={<Player />} />
                <Route path='login' element={<Login />} />
                <Route path='register' element={<Register />} />
                <Route path='id' element={<Id />} />
            </Route>
        </Routes>
    </BrowserRouter>
}