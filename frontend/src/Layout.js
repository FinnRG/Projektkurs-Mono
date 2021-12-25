import { Box } from 'react-bulma-components';
import { Outlet } from 'react-router-dom';
import Header from './components/shared/Header';

export const Layout = () => {

  return (
    <div className='App'>
      <Header />
      <Box>
        <Outlet />
      </Box>
    </div>
  );
}

