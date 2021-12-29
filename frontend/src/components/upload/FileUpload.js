import axios from 'axios';
import { useState } from 'react';
import { Form } from 'react-bulma-components';
const { InputFile } = Form;

const FileUpload = () => {

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

    return <main>
        <InputFile value={file} onChange={(e) => {
            setFile(e.target.files);
            upload(e.target.files.item(0));
        }} />
    </main>
}

export default FileUpload;
