import ShakaPlayer from 'shaka-player-react';
import 'shaka-player-react/dist/controls.css';
import { useNavigate, useParams } from 'react-router-dom';
import { Container, Skeleton, Stack, Title, Text } from '@mantine/core';
import { useQuery } from 'react-query';
import { VideoServiceClient } from '../gen/videos/v1/videos.client';
import { transport } from '../client';
import { useDocumentTitle } from '@mantine/hooks';
import dayjs from 'dayjs';

const Player = () => {
  const params = useParams();
  if (params.videoId == undefined) {
    const navigate = useNavigate();
    navigate('/videos');
  }

  const { isLoading, isError, data, error } = useQuery(
    ['video', params.videoId],
    () => {
      if (params.videoId == undefined) {
        return;
      }

      const videoService = new VideoServiceClient(transport());
      return videoService.getVideo({ id: params.videoId }).response;
    }
  );

  useDocumentTitle(data?.video?.title != undefined ? data.video.title : '');

  return (
    <Container size={'md'}>
      <Stack>
        <ShakaPlayer
          src={`http://msostream.io/stream/get/${params.videoId}/hls.m3u8`}
        />
        <Skeleton visible={isLoading}>
          <Title order={2}>{data?.video?.title}</Title>
          <Text color={'dimmed'}>{formatDate(data?.video?.date)}</Text>
          <Text>{data?.video?.description}</Text>
        </Skeleton>
      </Stack>
    </Container>
  );
};

const formatDate = (date: string | undefined) => {
  if (date == undefined) {
    return '';
  }
  const str = date.replace(' ', 'T').split(' ')[0];
  return dayjs(str).format('YYYY/MM/DD');
};

export default Player;
