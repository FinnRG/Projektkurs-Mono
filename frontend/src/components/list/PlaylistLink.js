import { Card } from 'react-bulma-components';
import { Link } from 'react-router-dom';

const { Content } = Card;

const PlaylistLink = (props) => {
    return (
        <>
            <Card>
                <Content>
                    <Link to={`/playlist/${props.id}`}>{props.title} </Link>
                </Content>
            </Card>
        </>
    );
};

export default PlaylistLink;