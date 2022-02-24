import { Box } from 'react-bulma-components';
import { Outlet } from 'react-router-dom';
import Header from './components/shared/Header';
import { ToastContainer } from 'react-toastify';

import 'react-toastify/dist/ReactToastify.min.css';

export const Layout = (props) => {
  return (
    <div className='App'>
      <Box>
        <Header />
        <Outlet />
        <ToastContainer />
      </Box>
    </div>
  );
};
