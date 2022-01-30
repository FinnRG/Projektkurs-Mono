import { useEffect, useState } from 'react';
import VideoLink from '../components/list/VideoLink';
import client from '../global/client';

const VideoList = () => {
  const [list, setList] = useState([]);

  useEffect(() => {
    const getVideoList = async () => {
      const resp = await client.get('/video/list');
      setList(resp.data);
    };
    getVideoList();
  }, []);

  return (
    <>
      {list.length === 0 && <p>There are currently no uploaded videos :(</p>}
      {list.length > 0 &&
        list.map((video, index) => (
          <VideoLink key={index} title={video.title} id={video.id} />
        ))}
    </>
  );
};

export default VideoList;
