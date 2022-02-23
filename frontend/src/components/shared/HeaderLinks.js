import { useContext } from 'react';
import { Link } from 'react-router-dom';
import { Box, Navbar } from 'react-bulma-components';
import client from '../../global/client';
import userContext from '../../global/userContext';
import HeaderLink from './HeaderLink';

const HeaderLinks = () => {
  const user = useContext(userContext);
  const handleLogout = () => {
    client
      .post('/user/logout')
      .then(() => {
        user.setUserId(null);
        user.setLoggedIn(false);
      })
      .catch((err) => {
        console.log(err);
      });
  };

  return (
    <>
        <HeaderLink text='Player' />
        <HeaderLink text='Videos' />
      {user.loggedIn && (
        <>
          <HeaderLink text='Upload' />
          <HeaderLink text='Playlist' />
          <HeaderLink text='Edit tags' to='tag/edit' />
        </>
      )}
      {!user.loggedIn && (
        <>
            <HeaderLink text='Login' />
            <HeaderLink text='Register' />
        </>
      )}
      {user.loggedIn && (
        <>
        <HeaderLink text='Id' />
        <HeaderLink text='Logout' to='player' onClick={(e) => handleLogout(e)} />
        </>
      )}
    </>
  );
};

export default HeaderLinks;
