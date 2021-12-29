import { faEnvelope, faLock } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useState } from 'react';
import { Button, Form, Icon } from 'react-bulma-components';
import { client } from '../App';
import InputFormField from '../components/shared/InputFormField';

const { Field, Control } = Form;

const Login = () => {
    const [email, setEmail] = useState('');
    const [success, setSuccess] = useState(false);
    const [password, setPassword] = useState('');

    return <form onSubmit={(e) => {
        client({
            method: 'post',
            url: 'http://localhost:8000/user/login',
            params: {
                email,
                password
            },
            withCredentials: true,
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
        })
            .then((resp) => {
                setSuccess(true);
            })
            .catch((err) => console.log(err))
        e.preventDefault();
    }}>
        <Field>
            <InputFormField label={"Email"} type={"email"} value={email} setValue={setEmail} icon={faEnvelope} />
            <InputFormField label={"Password"} type={"password"} value={password} setValue={setPassword} icon={faLock} />
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