import {
  Anchor,
  Container,
  LoadingOverlay,
  Paper,
  Text,
  Title,
} from '@mantine/core';
import React, { useContext, useState } from 'react';
import LoginForm from '../components/Login/LoginForm';
import RegisterForm from '../components/Login/RegisterForm';
import UserContext from '../context/userContext';

const Login = () => {
  const [showLoginForm, setShowLoginForm] = useState(true);
  const [loading, setLoading] = useState(false);
  const { setUser } = useContext(UserContext);

  const switchToRegister = (event: React.MouseEvent) => {
    setShowLoginForm(!showLoginForm);
    event.preventDefault();
  };

  // TODO: Properly implement schema checking using zod
  // TODO: Error fetching
  const onError = () => {};

  return (
    <Container size={420} my={40}>
      <LoadingOverlay visible={loading} />
      <Title
        align='center'
        sx={(theme) => ({
          fontFamily: `Greycliff CF, ${theme.fontFamily}`,
          fontWeight: 900,
        })}
      >
        Welcome back!
      </Title>
      <Text color='dimmed' size='sm' align='center' mt={5}>
        {showLoginForm
          ? 'Do not have an account yet? '
          : 'Already have an account? '}
        <Anchor<'a'>
          href='#'
          size='sm'
          onClick={(event) => switchToRegister(event)}
        >
          {showLoginForm ? 'Create account' : 'Login'}
        </Anchor>
      </Text>

      <Paper withBorder shadow='md' p={30} mt={30} radius='md'>
        {showLoginForm ? (
          <LoginForm
            onSuccess={(jwt) => setUser && setUser(jwt)}
            setLoading={setLoading}
            onError={onError}
          />
        ) : (
          <RegisterForm setLoading={setLoading} onError={onError}
            onSuccess={(jwt) => setUser && setUser(jwt)}
           />
        )}
      </Paper>
    </Container>
  );
};

export default Login;
