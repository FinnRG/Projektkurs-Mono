import { faEnvelope, faLock } from '@fortawesome/free-solid-svg-icons';
import { useState } from 'react';
import { Form } from 'react-bulma-components';
import { Navigate } from 'react-router';
import { client } from '../App';
import FormInputField from '../components/shared/FormInputField';
import FormSubmitButton from '../components/shared/FormSubmitButton';

const { Field } = Form;

const Login = () => {
    const [email, setEmail] = useState('');
    const [success, setSuccess] = useState(false);
    const [password, setPassword] = useState('');

    const handleSubmit = (e) => {
        client({
            method: 'post',
            url: 'http://localhost:8000/user/login',
            params: {
                email,
                password
            },
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
        })
            .then((resp) => {
                setSuccess(true);
            })
            .catch((err) => console.log(err))
        e.preventDefault();
    }

    return <form>
        <Field>
            <FormInputField label={"Email"} type={"email"} value={email} setValue={setEmail} icon={faEnvelope} />
            <FormInputField label={"Password"} type={"password"} value={password} setValue={setPassword} icon={faLock} />
        </Field>

        <FormSubmitButton setters={[setEmail, setPassword]} submit={handleSubmit} />

        {success && (
            <Navigate to='/videos' replace />
        )}
    </form >
}

export default Login;