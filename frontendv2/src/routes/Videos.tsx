import {
  ActionIcon,
  Card,
  createStyles,
  Group,
  Stack,
  Text,
  TextInput,
} from '@mantine/core';
import { useDebouncedValue } from '@mantine/hooks';
import { useEffect, useState } from 'react';
import { useQuery } from 'react-query';
import { Link } from 'react-router-dom';
import { ArrowRight, Search } from 'tabler-icons-react';
import { getVideos, transport, Video } from '../client';
import { SearchVideosRequest } from '../gen/search/v1/search';
import { SearchServiceClient } from '../gen/search/v1/search.client';

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

  searchbar: {
    maxWidth: '320px',
    marginLeft: 'auto',
    marginRight: 'auto',
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
            <Text size='xs'>{video.author}</Text>
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
  const { classes } = useStyles();
  const [videos, setVideos] = useState<Video[]>([]);
  const [query, setQuery] = useState('');
  const [debounced] = useDebouncedValue(query, 200);

  const { data } = useQuery(['videos', debounced], () => {
    const searchService = new SearchServiceClient(transport());
    return searchService.searchVideos(SearchVideosRequest.create({query: debounced})).response;
  })

  return (
    <>
      <Stack>
        <TextInput
          icon={<Search size={18} />}
          className={classes.searchbar}
          radius='xl'
          size='md'
          style={{ maxWidth: '320px' }}
          rightSection={
            <ActionIcon size={32} radius='xl' variant='filled'>
              <ArrowRight size={18} />
            </ActionIcon>
          }
          onChange={(val) => setQuery(val.target.value)}
          placeholder='Search videos'
          rightSectionWidth={42}
        />
        {data?.videos.map((video, index) => (
          <VideoCard video={video} key={index} />
        ))}
      </Stack>
    </>
  );
};

export default Videos;
