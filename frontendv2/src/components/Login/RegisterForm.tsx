import { Anchor, Button, Checkbox, Group, TextInput } from '@mantine/core';
import { useInputState } from '@mantine/hooks';
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { register } from '../../client';
import PasswordStrength from './PasswordStrength';

interface RegisterFormProps {
  setLoading: React.Dispatch<React.SetStateAction<boolean>>;
  onSuccess: (arg0: string) => unknown;
}

const RegisterForm = ({ setLoading, onSuccess }: RegisterFormProps) => {
  const [name, setName] = useInputState('');
  const [email, setEmail] = useInputState('');
  const [emailErr, setEmailErr] = useState(false);
  const [password, setPassword] = useInputState('');
  const navigate = useNavigate();

  const onSubmit = () => {
    setLoading(true);
    register(name, email, password).then((jwt) => {
      setLoading(false);
      onSuccess(jwt);
      navigate('/videos');
    })
      .catch((res) => {
        if (res.response && res.response.status == 503) {
          setEmailErr(true);
        }
      });

    setLoading(false);
  };

  return (
    <>
      <TextInput
        value={name}
        onChange={setName}
        label='Name'
        placeholder='Your Name'
        required
      />
      <TextInput
        value={email}
        onChange={setEmail}
        error={emailErr ? 'Email already exists' : ''}
        label='Email'
        placeholder='you@mantine.dev'
        mt='md'
        required
      />
      <PasswordStrength mt='md' value={password} setValue={setPassword} />
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
        Register
      </Button>
    </>
  );
};

export default RegisterForm;
