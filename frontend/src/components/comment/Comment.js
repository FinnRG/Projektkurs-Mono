import { Content, Media } from 'react-bulma-components';
import { useParams } from 'react-router';
import client from '../../global/client';
import Trash from '../shared/Trash';

const { Item } = Media;

const Comment = (props) => {

    const params = useParams();

    const deleteComment = () => {
        client.post('/comment/delete', {}, {
            params: {
                comment_id: props.comment_id
            }
        }).then(() => props.onUpdate())
    }

    return <Media renderAs='article'>
        <Item align='center'>
            <Content>
                <p>
                    <strong>{props.username}</strong>
                    <br />
                    {props.content}
                    <br />
                    {/* TODO: Add like functionality here */}
                </p>
            </Content>
        </Item>
        <Item align='right' >
            <Trash onClick={deleteComment} />
        </Item>
    </Media>
}

export default Comment;