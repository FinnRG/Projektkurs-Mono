import { faAlignLeft, faHeading } from '@fortawesome/free-solid-svg-icons';
import { useEffect, useState } from 'react';
import { Button } from 'react-bulma-components';
import { useParams } from 'react-router';
import FormInputField from '../components/shared/FormInputField';
import client from '../global/client';

const EditVideo = () => {

    const params = useParams();
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');

    const handleSubmit = () => {
        client.post('/video/update', {}, {
            params: {
                video_id: params.video_id,
                title,
                description,
            }
        }).then(() => console.log('Video update successfull'));
    }

    useEffect(() => {
        const getVideoInfo = (() => {
            client.get('/video/get', { params: { video_id: params.video_id } })
                .then((resp) => {
                    setTitle(resp.data.title);
                });
        })

        getVideoInfo();
    }, [])

    return <>
        <FormInputField label={'Title'} type={'text'} value={title} setValue={setTitle} icon={faHeading} />
        <FormInputField label={'Description'} type={'text'} value={description} setValue={setDescription} icon={faAlignLeft} />
        <Button mt={2} onClick={() => handleSubmit()} color='primary'>Submit</Button>
    </>
}

export default EditVideo;