import {
  Anchor,
  Button,
  Checkbox,
  Group,
  PasswordInput,
  TextInput,
} from '@mantine/core';
import { useInputState } from '@mantine/hooks';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { login } from '../../client';

interface LoginFormProps {
  setLoading: (arg0: boolean) => void;
  onSuccess: (arg0: string) => unknown;
}

const LoginForm = ({ setLoading, onSuccess }: LoginFormProps) => {
  const [email, setEmail] = useInputState('');
  const [loginErr, setLoginErr] = useState(false);
  const [password, setPassword] = useInputState('');
  const navigate = useNavigate();

  const onSubmit = () => {
    setLoading(true);
    login(email, password)
      .then((jwt) => {
        setLoginErr(false);
        onSuccess(jwt);
        navigate('/videos');
      })
      .catch((res) => {
        if (res.response && res.response.status == 503) {
          setLoginErr(true);
        }
      });

    setLoading(false);
  };

  return (
    <>
      <TextInput
        value={email}
        onChange={setEmail}
        error={loginErr}
        label='Email'
        placeholder='you@mantine.dev'
        required
      />
      <PasswordInput
        value={password}
        onChange={setPassword}
        error={loginErr}
        label='Password'
        placeholder='Your password'
        required
        mt='md'
      />
      <Group position='apart' mt='md'>
        <Checkbox label='Remember me' />
        <Anchor<'a'>
          onClick={(event) => event.preventDefault()}
          href='#'
          size='sm'
        >
          Forgot password?
        </Anchor>
      </Group>
      <Button onClick={onSubmit} fullWidth mt='xl'>
        Sign in
      </Button>
    </>
  );
};

export default LoginForm;
