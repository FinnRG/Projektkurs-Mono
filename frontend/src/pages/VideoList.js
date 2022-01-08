import { useEffect, useState } from 'react';
import client from '../global/client';
import VideoLink from '../components/list/VideoLink';

const VideoList = () => {

    const [list, setList] = useState([]);

    useEffect(() => {
        const getVideoList = async () => {
            const resp = await client.get("http://localhost:8000/video/list");
            setList(resp.data);
        }
        getVideoList();
    }, []);

    return <>
        {list.map((video, index) => <VideoLink key={index} title={video.title} id={video.id} />)}
    </>
}

export default VideoList;