import { useState } from 'react';
import { Button, Form, Media } from 'react-bulma-components';
import { useParams } from 'react-router';
import { client } from '../../App';

const { Control, Field, Textarea } = Form;

const CreateComment = () => {

    const params = useParams();

    const [content, setContent] = useState('');
    const [loading, setLoading] = useState(false);

    const handleSubmit = (e) => {
        setLoading(true);
        client.post('http://localhost:8000/comment/create', {}, {
            params: {
                video_id: params.video_id,
                content: content
            },
            withCredentials: true
        })
            .then((_) => setLoading(false))
            .catch((err) => console.log(err));
    };

    return <Media>
        <Media.Item align='center'>
            <Field>
                <Control>
                    <Textarea onChange={(e) => setContent(e.target.value)} className={loading && ('is-loading')} size='medium' placeholder='Enter your comment' />
                </Control>
            </Field>
            <Field>
                <Control>
                    <Button onClick={(e) => handleSubmit(e)}>Post comment</Button>
                </Control>
            </Field>
        </Media.Item>
    </Media>
}

export default CreateComment;