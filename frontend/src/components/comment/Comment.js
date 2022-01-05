import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { Media, Content } from 'react-bulma-components';
import Trash from '../shared/Trash';
import { client } from '../../App';
import { useParams } from 'react-router';

const { Item } = Media;

const Comment = (props) => {

    const params = useParams();

    const deleteComment = () => {
        client.post('http://localhost:8000/comment/delete', {}, {
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