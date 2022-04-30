import ShakaPlayer from 'shaka-player-react';
import 'shaka-player-react/dist/controls.css';
import { useParams } from 'react-router-dom';

const Player = () => {
  const params = useParams();
  return (
    <ShakaPlayer
      src={`http://msostream.io/stream/get/${params.videoId}/hls.m3u8`}
    />
  );
};

export default Player;
