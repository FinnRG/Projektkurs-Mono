import { faAlignLeft, faHeading, faPlus, faTag } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useEffect, useState } from 'react';
import { Button, Columns, Form, Panel } from 'react-bulma-components';
import FormInputField from '../components/shared/FormInputField';
import client from '../global/client';

const { Field } = Form;

const EditTag = () => {

    const [tags, setTags] = useState([{ id: -1, name: 'Add a tag', deleted: false, description: '' }]);

    const [tagId, setTagId] = useState(null);
    const [name, setName] = useState('');
    const [description, setDescription] = useState('');

    useEffect(() => {
        const getTags = () => {
            client.get('/tag/get')
                .then((resp) => setTags([...tags, ...resp.data]));
        };

        getTags();
    }, []);

    const editTag = (tag) => {
        setTagId(tag.id)
        setName(tag.id === -1 ? '' : tag.name);
        setDescription(tag.description);
    };

    const handleCreate = () => {
        client.post('/tag/create', {}, {
            params: {
                tag_id: tagId,
                name,
                description,
            }
        }).then((resp) => setTags([...tags, resp.data]));
    }

    const handleSubmit = () => {

        if (tagId === -1) {
            handleCreate();
            return;
        }

        client.post('/tag/update', {}, {
            params: {
                tag_id: tagId,
                name,
                description,
            }
        }).then(() => console.log("Tag update successfull"));
    };

    const handleDelete = () => {
        client.post('/tag/delete', {}, {
            params: {
                tag_id: tagId
            }
        }).then(() => {
            setTags(tags.filter((tag) => tag.id !== tagId));
        });
    };

    return <Columns>
        <Columns.Column size='half' >
            <Panel color='info'>
                <Panel.Header>
                    Tags
                </Panel.Header>
                {tags.filter((tag) => tag.deleted === false)
                    .map((tag, index) => <Panel.Block className={'has-icons-left'} active={tagId == tag.id} key={index} onClick={() => editTag(tag)} >
                        <Panel.Icon>
                            <FontAwesomeIcon icon={tag.id === -1 ? faPlus : faTag} />
                        </Panel.Icon>
                        {tag.name}
                    </Panel.Block>)}
            </Panel>
        </Columns.Column>
        <Columns.Column size='half' >
            <FormInputField label={'Name'} type={'text'} value={name} setValue={setName} icon={faHeading} />
            <FormInputField label={'Description'} type={'text'} value={description} setValue={setDescription} icon={faAlignLeft} />

            <Button.Group mt={2}>
                <Button onClick={() => handleSubmit()} color='primary'> Submit </Button>
                <Button onClick={() => handleDelete()} color='danger'>Delete</Button>
            </Button.Group>
        </Columns.Column>
    </Columns>
};

export default EditTag;