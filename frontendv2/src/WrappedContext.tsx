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
import CreateVideo from './routes/Create';
import Login from './routes/Login';
import Player from './routes/Player';
import Videos from './routes/Videos';
import { ReactQueryDevtools } from 'react-query/devtools';
import { NotificationsProvider } from '@mantine/notifications';

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
          <NotificationsProvider>
            <QueryClientProvider client={queryClient}>
              <BrowserRouter>
                <Routes>
                  <Route path='/' element={<Context />}>
                    <Route path='player' element={<Player />}>
                      <Route path=':videoId' element={<Player />} />
                    </Route>
                    <Route path='videos' element={<Videos />} />
                    <Route path='login' element={<Login />} />
                    <Route path='create' element={<CreateVideo />} />
                  </Route>
                </Routes>
              </BrowserRouter>
              <ReactQueryDevtools initialIsOpen={false} />
            </QueryClientProvider>
          </NotificationsProvider>
        </MantineProvider>
      </ColorSchemeProvider>
    </UserContext.Provider>
  );
};

export default WrappedContext;
