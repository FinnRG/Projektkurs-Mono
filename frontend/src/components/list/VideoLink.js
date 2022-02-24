import { Card, Columns } from 'react-bulma-components';
import { Link } from 'react-router-dom';
import Trash from '../shared/Trash';

const VideoLink = (props) => {
  return (
    <>
      <Card>
        <Card.Content>
          <Columns>
            {props.tagView && (
              <Columns.Column className='is-narrow'>
                <Trash onClick={() => props.trash()} />
              </Columns.Column>
            )}
            <Columns.Column>
              <Link className='is-size-5' to={`/player/${props.id}`}>
                {props.title}{' '}
              </Link>
              <br />
              {props.description}
            </Columns.Column>
          </Columns>
        </Card.Content>
      </Card>
    </>
  );
};

export default VideoLink;
