import { Card } from 'react-bulma-components';
import { Link } from 'react-router-dom';
import Trash from '../shared/Trash';

const { Content } = Card;

const VideoLink = (props) => {
  return (
    <>
      <Card>
        <Content>
          {props.tagView && <Trash onClick={() => props.trash()} />}
          <Link className="ml-4" to={`/player/${props.id}`}>
            {props.title}{' '}
          </Link>
        </Content>
      </Card>
    </>
  );
};

export default VideoLink;
