import { useEffect, useState } from 'react';
import VideoLink from '../components/list/VideoLink';
import client from '../global/client';

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