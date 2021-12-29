import { Form, Button } from 'react-bulma-components';

const { Field, Control } = Form;

const FormSubmitButton = (props) => {
    return <Field kind='group'>
        <Control>
            <Button color='link' onClick={(e) => props.submit(e)}>Submit</Button>
        </Control>
        <Control>
            <Button color='link' colorVariant='light' onClick={() => {
                props.setters.forEach(func => func(''));
            }
            }>
                Cancel
            </Button>
        </Control>
    </Field>
}

export default FormSubmitButton;