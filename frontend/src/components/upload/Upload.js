import { Form } from 'react-bulma-components';
import { useState } from 'react';
import axios from 'axios';
const { InputFile } = Form;

const Upload = () => {

    let [file, setFile] = useState(null);

    const upload = (file) => {
        axios.post('http://localhost:8000/upload/' + file.name, file, {
            headers: {
                'Content-Type': file.type,
                'Authorization': 'Accept',
                'Access-Control-Allow-Origin': true,
            }
        })
    }

    return <>
        <InputFile value={file} onChange={(e) => {
            setFile(e.target.files);
            upload(e.target.files.item(0));
        }} />
    </>
}

export default Upload;
