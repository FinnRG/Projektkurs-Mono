import { Box } from 'react-bulma-components';
import { Outlet } from 'react-router-dom';
import Header from './components/shared/Header';

export const Layout = (props) => {
  return (
    <div className='App'>
      <Box>
        <Header />
        <Outlet />
      </Box>
    </div>
  );
};
