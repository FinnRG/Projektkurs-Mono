import {
  Anchor,
  Button,
  Checkbox,
  Group,
  PasswordInput,
  TextInput,
} from '@mantine/core';
import { useInputState } from "@mantine/hooks";
import { useState } from 'react';
import { register } from "../../client";
import PasswordStrength from "./PasswordStrength";

interface RegisterFormProps {
  setLoading: Function;
  onError: Function;
  onSuccess: (arg0: string) => any;
}

const RegisterForm = ({ setLoading, onSuccess, onError }: RegisterFormProps) => {
  const [name, setName] = useInputState('');
  const [email, setEmail] = useInputState('');
  const [password, setPassword] = useInputState('');

  const onSubmit = () => {
    setLoading(true);    
    register(email, password)
      .then((jwt) => {
        setLoading(false);
        onSuccess(jwt);
      })
      .catch((_) => onError());
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
        label='Email'
        placeholder='you@mantine.dev'
        mt='md'
        required
      />
      <PasswordStrength mt="md" value={password} setValue={setPassword} />
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
