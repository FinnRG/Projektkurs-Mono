import { useEffect, useState } from 'react';
import { Container } from 'react-bulma-components';
import { useParams } from 'react-router';
import client from '../../global/client';
import ExtendedViewPanel from './ExtendedViewPanel';
import TagView from './TagView';

const ExtendedView = () => {
    const params = useParams();

    const [tags, setTags] = useState([])

    const getTags = () => {
        client.get("/tag/get", {
            params: {
                video_id: params.video_id
            }
        })
            .then((resp) => {
                setTags(resp.data);
            });
    };

    useEffect(() => {
        getTags();
    }, []);

    return <Container>
        <TagView tags={tags} setTags={setTags} />
        <ExtendedViewPanel update={() => getTags()} />
    </Container>
}

export default ExtendedView;
