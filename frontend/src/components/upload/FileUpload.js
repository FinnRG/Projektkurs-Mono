import { Form } from 'react-bulma-components';
const { InputFile } = Form;

const FileUpload = (props) => {

    return <InputFile onChange={(e) => {
        console.log(e.target.files);
        props.setValue(e.target.files[0]);
    }} />
}

export default FileUpload;