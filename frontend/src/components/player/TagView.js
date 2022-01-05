import { faTrash } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useState } from 'react';
import { Panel } from 'react-bulma-components';
import { useParams } from 'react-router';
import { useEffect } from 'react/cjs/react.development';
import { client } from '../../App';
import Trash from '../shared/Trash';
import Tag from '../tag/Tag';

const { Block, Header, Icon } = Panel;

const TagView = () => {

    const params = useParams();

    const [tags, setTags] = useState([])

    useEffect(() => {
        const getTags = () => {
            client.get("http://localhost:8000/tag/get", {
                params: {
                    video_id: params.video_id
                }
            })
                .then((resp) => {
                    setTags(resp.data);
                })
                .catch((err) => console.log(err));
        };

        getTags();
    }, []);

    const removeTag = (id) => {
        setTags(tags.filter((tag) => tag.id !== id));
    }

    return <Panel color='info'>
        <Header>
            Tags
        </Header>
        {tags.map((tag, index) => <Tag key={index} name={tag.name} tag_id={tag.id} onSuccess={() => removeTag(tag.id)} />)}
    </Panel>
}

export default TagView;