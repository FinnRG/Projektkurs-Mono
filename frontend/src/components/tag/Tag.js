import { Tag } from 'react-bulma-components';
import { useParams } from 'react-router';
import client from '../../global/client';
import { Navigate } from 'react-router';
import { useState } from 'react';

const TagViewElement = (props) => {

    const params = useParams();

    const [redirect, setRedirect] = useState(false);

    const onDelete = () => {
        client({
            method: 'post',
            params: {
                tag_id: props.tag_id,
                video_id: params.video_id,
            },
            url: '/tag/remove',
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
        })
            .then(() => props.onSuccess())
            .catch((err) => console.log(err));
    }

    if (redirect) {
        return <Navigate to={`/tag/${props.tag_id}`} />;
    }

    return <Tag.Group hasAddons>
        <Tag onClick={() => setRedirect(true)} color='danger'>
            {props.name}
        </Tag>
        <Tag remove onClick={() => onDelete()} />
    </Tag.Group>
}

export default TagViewElement;