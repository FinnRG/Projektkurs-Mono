import { faPen } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useEffect, useState } from 'react';
import { Card, Content, Icon, Level } from 'react-bulma-components';
import { useParams } from 'react-router';
import { Navigate } from 'react-router';
import { Link } from 'react-router-dom';
import client from '../global/client';

const TagView = () => {
  const [tag, setTag] = useState({});
  const [videos, setVideos] = useState([]);
  const [redirect, setRedirect] = useState(false);
  const params = useParams();

  useEffect(() => {
    const getTag = () => {
      client
        .get('/tag/get', { params: { tag_id: params.tag_id } })
        .then((resp) => setTag(resp.data));
    };

    const getVideos = () => {
      client
        .get('/tag/videos', { params: { tag_id: params.tag_id } })
        .then((resp) => setVideos(resp.data));
    };

    getTag();
    getVideos();
  }, []);

  if (redirect == true) {
    return <Navigate to={`/tag/edit/${params.tag_id}`} />;
  }

  return (
    <>
      <Level className='mb-0'>
        <p className='title'>{tag.name}</p>
        <Icon onClick={() => setRedirect(true)} size='small'>
          <FontAwesomeIcon icon={faPen} />
        </Icon>
      </Level>
      <p>{tag.description}</p>
      <br />
      <br />

      {videos.length > 0 && (
        <p className='title is-size-5'>Videos with this tag:</p>
      )}
      {videos.map((video) => (
        <Card>
          <Card.Content>
            <Content>
              <Link to={`/player/${video.id}`}>{video.title}</Link>
            </Content>
          </Card.Content>
        </Card>
      ))}
    </>
  );
};

export default TagView;
