import { faPen } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useEffect, useState } from 'react';
import { Icon, Level } from 'react-bulma-components';
import { useParams } from 'react-router';
import { Navigate } from 'react-router';
import client from '../global/client';

const TagView = () => {

    const [tag, setTag] = useState({});
    const [redirect, setRedirect] = useState(false);
    const params = useParams();
    
    useEffect(() => {
        const getTag = () => {
            client.get(`/tag/get/${params.tag_id}`)
                .then((resp) => setTag(resp.data));
        }

        getTag();
    }, []);

    if (redirect == true) {
        return <Navigate to={`/tag/edit/${params.tag_id}`} />;
    }

    return <>
        <Level>
            <p className='title'>
                {tag.name}
            </p>
            <Icon onClick={() => setRedirect(true)} size='small'>
                <FontAwesomeIcon icon={faPen} />
            </Icon>
        </Level>
        <p>{tag.description}</p>
    </>
}

export default TagView;