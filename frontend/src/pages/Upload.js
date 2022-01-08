import { faAlignLeft, faHeading } from '@fortawesome/free-solid-svg-icons';
import { useState } from 'react';
import { Form, Progress } from 'react-bulma-components';
import FormInputField from '../components/shared/FormInputField';
import FormSubmitButton from '../components/shared/FormSubmitButton';
import FileUpload from '../components/upload/FileUpload';
import client from '../global/client';

const { Field } = Form;

const Upload = () => {

    const [file, setFile] = useState('');
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [success, setSuccess] = useState(false);
    const [progress, setProgress] = useState(null);


    const handleSubmit = (e) => {

        client.post(`http://localhost:8000/upload/${title}`, file, {
            params: {
                description,
            },
            headers: {
                'Content-Type': file.type,
            },
            // https://stackoverflow.com/questions/44936028/progress-bar-with-axios
            onUploadProgress: (progressEvent) => {
                const totalLength = progressEvent.lengthComputable ? progressEvent.total : progressEvent.target.getResponseHeader('content-length') || progressEvent.target.getResponseHeader('x-decompressed-content-length');
                if (totalLength !== null) {
                    setProgress(Math.round((progressEvent.loaded * 100) / totalLength));
                }
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
        {progress && (
            <Progress max={100} value={progress} />
        )}
    </form>
}


export default Upload;