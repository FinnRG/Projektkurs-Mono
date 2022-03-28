import { Card, createStyles, Group, Text } from '@mantine/core';
import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { getVideos, Video } from '../client';

const useStyles = createStyles((theme) => ({
  card: {
    backgroundColor:
      theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
  },

  title: {
    fontWeight: 700,
    fontFamily: `Greycliff CF, ${theme.fontFamily}`,
    lineHeight: 1.2,
  },

  body: {
    padding: theme.spacing.md,
  },
}));

interface VideoCardProps {
  video: Video;
}

// TODO: Add Avatar + Thumbnail + swap video.id to video.created_at
const VideoCard = ({ video }: VideoCardProps) => {
  const { classes } = useStyles();

  return (
    <Card<typeof Link>
      withBorder
      radius='md'
      p={0}
      className={classes.card}
      component={Link}
      to={`/player/${video.id}`}
    >
      <Group noWrap spacing={0}>
        <div className={classes.body}>
          <Text className={classes.title} transform='uppercase' mt='xs' mb='md'>
            {video.title}
          </Text>
          <Group noWrap spacing='xs'>
            <Text size='xs'>{video.user_id}</Text>
            <Text size='xs' color='dimmed'>
              •
            </Text>
            <Text size='xs' color='dimmed'>
              {video.id}
            </Text>
          </Group>
        </div>
      </Group>
    </Card>
  );
};

const Videos = () => {
  const [videos, setVideos] = useState<Video[]>([]);

  useEffect(() => {
    getVideos(setVideos);
  }, []);

  return (
    <>
      {videos.map((video, index) => (
        <VideoCard video={video} key={index} />
      ))}
    </>
  );
};

export default Videos;
