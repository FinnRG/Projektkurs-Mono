import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faTrash } from '@fortawesome/free-solid-svg-icons';
import { Media, Content } from 'react-bulma-components';
import { useState } from 'react';
import { client } from '../../App';
import { useParams } from 'react-router';

const { Item } = Media;

const Comment = (props) => {

    const [trashClassName, setTrashClassName] = useState('has-text-grey-light');
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
            <FontAwesomeIcon
                onMouseEnter={() => setTrashClassName('has-text-dark')}
                onMouseLeave={() => setTrashClassName('has-text-grey-light')}
                onClick={() => deleteComment()}
                className={trashClassName}
                icon={faTrash} />
        </Item>
    </Media>
}

export default Comment;