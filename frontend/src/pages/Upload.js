import { faAlignLeft, faHeading } from '@fortawesome/free-solid-svg-icons';
import { useState } from 'react';
import { Form } from 'react-bulma-components';
import FormInputField from '../components/shared/FormInputField';
import FileUpload from '../components/upload/FileUpload';

const { Field } = Form;

const Upload = () => {

    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');

    return <form>
        <Field>
            <FormInputField label={"Title"} value={title} setValue={setTitle} icon={faHeading} />
            <FormInputField label={"Description"} value={description} setValue={setDescription} icon={faAlignLeft} />
        </Field>
        <Field className="mb-1">
            <FileUpload />
        </Field>
    </form>
}


export default Upload;