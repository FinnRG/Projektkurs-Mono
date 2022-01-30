import { Card } from 'react-bulma-components';
import { Link } from 'react-router-dom';

const { Content } = Card;

const VideoLink = (props) => {
  return (
    <>
      <Card>
        <Content>
          <Link to={`/player/${props.id}`}>{props.title} </Link>
        </Content>
      </Card>
    </>
  );
};

export default VideoLink;
