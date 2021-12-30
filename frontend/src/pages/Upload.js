import { faAlignLeft, faHeading } from '@fortawesome/free-solid-svg-icons';
import { useState } from 'react';
import { Form } from 'react-bulma-components';
import FormInputField from '../components/shared/FormInputField';
import FormSubmitButton from '../components/shared/FormSubmitButton';
import FileUpload from '../components/upload/FileUpload';
import { client } from '../App';

const { Field } = Form;

const Upload = () => {

    const [file, setFile] = useState('');
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [success, setSuccess] = useState(false);

    const handleSubmit = (e) => {

        client.post(`http://localhost:8000/upload/${title}`, file, {
            withCredentials: true,
            params: {
                description,
            },
            headers: {
                'Content-Type': file.type,
            }
        })
            .then((_) => setSuccess(true))
            .catch((err) => console.log(err))
        e.preventDefault();
    }

    return <form>
        <Field>
            <FormInputField label={"Title"} value={title} setValue={setTitle} icon={faHeading} />
            <FormInputField label={"Description"} value={description} setValue={setDescription} icon={faAlignLeft} />
        </Field>
        <Field className="mb-1">
            <FileUpload setValue={setFile} />
        </Field>
        <FormSubmitButton submit={handleSubmit} setters={[setFile, setTitle, setDescription]} />
    </form>
}


export default Upload;