import { useState } from 'react';
import { Navigate } from 'react-router';
import VideoLink from './VideoLink';

const VideoEntry = (props) => {
  const [id, setId] = useState(null);

  return (
    <>
      <VideoLink
        title={props.title}
        onClick={(e) => {
          setId(props.id);
        }}
      />

      {id && <Navigate to={`/player${id}`} />}
    </>
  );
};

export default VideoEntry;
