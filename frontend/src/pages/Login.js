import { faEnvelope, faLock } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useState } from 'react';
import { Button, Form, Icon } from 'react-bulma-components';
import { client } from '../App';

const { Input, Field, Label, Control } = Form;

const Login = () => {
    const [email, setEmail] = useState('');
    const [success, setSuccess] = useState(false);
    const [password, setPassword] = useState('');

    return <form onSubmit={(e) => {
        let BodyFormData = new FormData();
        BodyFormData.append('email', email);
        BodyFormData.append('password', password);
        client({
            method: 'post',
            url: 'http://localhost:8000/user/login',
            data: BodyFormData,
            withCredentials: true,
            headers: { 'Content-Type': 'multipart/form-data' }
        })
            .then((resp) => {
                setSuccess(true);
            })
            .catch((err) => console.log(err))
        e.preventDefault();
    }}>
        <Field>
            <Label>Email</Label>
            <Control>
                <Input
                    value={email}
                    onChange={(e) => {
                        return setEmail(e.target.value);
                    }} />
                <Icon align='left' size='small'>
                    <FontAwesomeIcon icon={faEnvelope} />
                </Icon>
            </Control>
            <Label>Password</Label>
            <Control>
                <Input
                    value={password}
                    onChange={(e) => {
                        return setPassword(e.target.value);
                    }} />
                <Icon align='left' size='small'>
                    <FontAwesomeIcon icon={faLock} />
                </Icon>
            </Control>
        </Field>

        <Field kind='group'>
            <Control>
                <Button color='link'>Submit</Button>
            </Control>
            <Control>
                <Button color='link' colorVariant='light'>
                    Cancel
                </Button>
            </Control>
        </Field>
    </form>
}

export default Login;