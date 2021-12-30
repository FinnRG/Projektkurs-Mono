import axios from 'axios';
import { wrapper } from 'axios-cookiejar-support';
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { CookieJar } from 'tough-cookie';
import Upload from './pages/Upload';
import { Layout } from './Layout';
import Id from './pages/Id';
import Login from './pages/Login';
import Register from './pages/Register';
import VideoList from './pages/VideoList';
import PlayerPage from './pages/PlayerPage';
import Player from './components/player/Player';

const jar = new CookieJar();
export const client = wrapper(axios.create({ jar }));

export const App = () => {

    return <BrowserRouter>
        <Routes>
            <Route path='/' element={<Layout />}>
                <Route path='player' element={<PlayerPage />} >
                    <Route index element={<main><p>Select a video</p></main>} />
                    <Route path=':video_id' element={<Player />} />
                </Route>
                <Route path='videos' element={<VideoList />} />
                <Route path='upload' element={<Upload />} />
                <Route path='login' element={<Login />} />
                <Route path='register' element={<Register />} />
                <Route path='id' element={<Id />} />
            </Route>
        </Routes>
    </BrowserRouter>
}