import ReactPlayer from 'react-player';
import { useParams } from 'react-router-dom';

const Player = () => {
  const params = useParams();
  return (
    <ReactPlayer
      url={`http://localhost:4500/get/${params.videoId}`}
      controls={true}
      config={{ file: { forceHLS: true } }}
    />
  );
};

export default Player;
