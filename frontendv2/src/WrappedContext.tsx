import {
  ColorScheme,
  ColorSchemeProvider,
  MantineProvider,
} from '@mantine/core';
import { useColorScheme, useLocalStorage } from '@mantine/hooks';
import { useContext, useState } from 'react';
import { QueryClient, QueryClientProvider } from 'react-query';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import Context from './Context';
import UserContext from './context/userContext';
import Login from './routes/Login';
import Player from './routes/Player';
import Videos from './routes/Videos';

const WrappedContext = () => {
  const preferredColorScheme = useColorScheme();
  const [colorScheme, setColorScheme] = useLocalStorage<ColorScheme>({
    key: 'mantine-color-scheme',
    defaultValue: preferredColorScheme,
  });
  const toggleColorScheme = (value?: ColorScheme) =>
    setColorScheme(value || (colorScheme === 'dark' ? 'light' : 'dark'));
  const queryClient = new QueryClient();
  const userContext = useContext(UserContext);
  const [user, setUser] = useState(userContext.user);

  return (
    <UserContext.Provider value={{ user, setUser }}>
      <ColorSchemeProvider
        colorScheme={colorScheme}
        toggleColorScheme={toggleColorScheme}
      >
        <MantineProvider theme={{ colorScheme }} withGlobalStyles>
          <QueryClientProvider client={queryClient}>
            <BrowserRouter>
              <Routes>
                <Route path='/' element={<Context />}>
                  <Route path='player/:videoId' element={<Player />} />
                  <Route path='videos' element={<Videos />} />
                  <Route path='login' element={<Login />} />
                </Route>
              </Routes>
            </BrowserRouter>
          </QueryClientProvider>
        </MantineProvider>
      </ColorSchemeProvider>
    </UserContext.Provider>
  );
};

export default WrappedContext;
