import React, { useEffect, useState } from 'react';
import { BrowserRouter, Route, Routes } from "react-router-dom";
import CommentView from './components/player/CommentView';
import Player from './components/player/Player';
import { Layout } from './Layout';
import Id from './pages/Id';
import Login from './pages/Login';
import PlayerPage from './pages/PlayerPage';
import Register from './pages/Register';
import Upload from './pages/Upload';
import VideoList from './pages/VideoList';
import client from './global/client';
import userContext from './global/userContext';

export const App = () => {

    const [loggedIn, setLoggedIn] = useState(false);
    const [userId, setUserId] = useState(null);

    useEffect(() => {
        const getId = () => {
            client.get('http://localhost:8000/user/id', {
                headers: {
                    'Accept': 'text/html',
                    'Content-Type': 'text/plain',
                }
            })
                .then((resp) => setUserId(resp.data))
                .catch(() => null);
        }
        getId();

    })

    const user = {
        loggedIn,
        setLoggedIn,
        userId,
        setUserId,
    }

    return <BrowserRouter>
        <Routes>
            <Route path='/' element={<userContext.Provider value={user}>
                <Layout />
            </userContext.Provider>}>
                <Route path='player' element={<PlayerPage />} >
                    <Route index element={<main><p>Select a video</p></main>} />
                    <Route path=':video_id' element={<><Player /><CommentView /></>} />
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