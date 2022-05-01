import ShakaPlayer from 'shaka-player-react';
import 'shaka-player-react/dist/controls.css';
import { useParams } from 'react-router-dom';
import { Container } from '@mantine/core';

const Player = () => {
  const params = useParams();
  return (
    <Container size={'md'}>
      <ShakaPlayer
        src={`http://msostream.io/stream/get/${params.videoId}/hls.m3u8`}
      />
    </Container>
  );
};

export default Player;
