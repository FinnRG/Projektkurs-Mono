import { faEnvelope, faLock, faUser } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useState } from 'react';
import { Button, Form, Icon, Progress } from 'react-bulma-components';
import { Navigate } from 'react-router-dom';
import zxcvbn from 'zxcvbn';
import { client } from '../App';
import InputFormField from '../components/shared/InputFormField';

const { Field, Label, Control, Input, Help } = Form;

const calculatePasswordColor = (passwordStrength) => {
    switch (passwordStrength) {
        case 1:
            return 'danger';
        case 2:
            return 'warning';
        case 3:
            return 'success';
        case 4:
            return 'primary';

        default:
            return 'danger';
    }

}

const Register = () => {

    const [username, setUsername] = useState('testUser');

    const [email, setEmail] = useState('testEmail@email.com');
    const [password, setPassword] = useState('testPassword');
    const [success, setSuccess] = useState(null);

    const passwordStrength = zxcvbn(password).score;

    const passwordColor = calculatePasswordColor(passwordStrength);

    return <form onSubmit={(e) => {
        let BodyFormData = new FormData();
        BodyFormData.append('name', username);
        BodyFormData.append('email', email);
        BodyFormData.append('password', password);
        client({
            method: 'post',
            url: 'http://localhost:8000/user/register',
            params: {
                username,
                email,
                password
            },
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
            <InputFormField label={"Username"} value={username} setValue={setUsername} icon={faUser} />
            <InputFormField label={"Email"} type={"email"} value={email} setValue={setEmail} icon={faEnvelope} />
            <InputFormField label={"Password"} type={"password"} value={password} setValue={setPassword} icon={faLock} >
                <Progress className='mb-0' max={4} value={passwordStrength} size='small' color={passwordColor} />
                <Help color={passwordColor}>
                    {password.length > 0 && (passwordStrength <= 2 ? 'Password ist nicht stark genug!' : 'Password ist stark!')}
                </Help>
            </InputFormField>
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

        {success && (
            <Navigate to='/player' replace />
        )}
    </form>
}

export default Register;