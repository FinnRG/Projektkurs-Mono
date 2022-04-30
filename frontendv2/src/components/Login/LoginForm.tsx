import {
  Anchor,
  Button,
  Checkbox,
  Group,
  PasswordInput,
  TextInput,
} from '@mantine/core';
import { useInputState } from '@mantine/hooks';
import React from 'react';
import { login } from '../../client';

interface LoginFormProps {
  setLoading: (arg0: boolean) => void;
  onError: () => void;
  onSuccess: (arg0: string) => unknown;
}

const LoginForm = ({ setLoading, onError, onSuccess }: LoginFormProps) => {
  const [email, setEmail] = useInputState('');
  const [password, setPassword] = useInputState('');

  const onSubmit = () => {
    setLoading(true);
    login(email, password)
      .then((jwt) => {
        setLoading(false);
        onSuccess(jwt);
      })
      .catch(() => onError());
  };

  return (
    <>
      <TextInput
        value={email}
        onChange={setEmail}
        label='Email'
        placeholder='you@mantine.dev'
        required
      />
      <PasswordInput
        value={password}
        onChange={setPassword}
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
