import { useEffect, useState } from "react";
import { useParams } from "react-router";
import { useSearchParams } from "react-router-dom";
import VideoLink from "../components/list/VideoLink";
import client from "../global/client";

const PlaylistPage = () => {
    const params = useParams();
    const [playlistInfo, setPlaylistInfo] = useState(null);
    const [videos, setVideos] = useState(null);

    useEffect(() => {
        client({
            method: 'GET',
            url: '/playlist/info',
            params: {
                playlist_id: params.playlist_id
            }
        }).then((resp) => setPlaylistInfo(resp.data));

        client({
            method: 'GET',
            url: '/playlist/get',
            params: {
                playlist_id: params.playlist_id
            }
        }).then((resp) => setVideos(resp.data));
    }, [])

    const removeVideo = (entry_id) => {
        client({
            method: 'POST',
            url: '/playlist/remove',
            params: {
                playlist_id: params.playlist_id,
                entry_id
            }
        }).then(() => setVideos(videos.filter(({ id }) => id != entry_id)))
    }

    return <>
        {!playlistInfo && (<p>Fetching playlist Info</p>)}
        {playlistInfo && (
            <>
                <h1 className="title">{playlistInfo.title}</h1>
                {videos && videos.map(({ id, title }, index) => (
                    <VideoLink tagView trash={() => removeVideo(id)} id={id} title={title} key={index} />
                ))}
            </>
        )}
    </>
}

export default PlaylistPage;