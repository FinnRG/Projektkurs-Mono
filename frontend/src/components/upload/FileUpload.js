import { Form } from 'react-bulma-components';
const { InputFile } = Form;

const FileUpload = (props) => {

    return <InputFile value={props.value} onChange={(e) => {
        props.setValue(e.target.files);
    }} />
}

export default FileUpload;