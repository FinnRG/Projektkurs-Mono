import Cookies from 'js-cookie';
import React from 'react';
import client from '../client';

const jwt = localStorage.getItem('msostream-user');

interface UserContextProps {
  user: string | null;
  setUser?: React.Dispatch<React.SetStateAction<string | null>>;
}
const UserContext = React.createContext<UserContextProps>({ user: jwt });

const logout = () => {
  localStorage.removeItem('msostream-user');
  client.defaults.headers.common['authorization'] = '';
  window.location.reload();
};

export default UserContext;
export { logout };
