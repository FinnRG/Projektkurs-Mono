import { useEffect, useState } from 'react';
import { Box, Navbar } from 'react-bulma-components';
import HeaderLinks from './HeaderLinks';
import { useLocation } from 'react-router';

const Header = () => {

  const [showMenu, setShowMenu] = useState(false);

  const location = useLocation();

  useEffect(() => {
    setShowMenu(false);
  }, [location]);

  return (
    <>
      <Navbar mb={showMenu? 4 : 0} transparent={true}>
        <Navbar.Brand>
          <Navbar.Burger onClick={() => setShowMenu(!showMenu)} />
        </Navbar.Brand>

        <Navbar.Menu className={showMenu ? 'is-active' : ''}>
          <Navbar.Container>
            <HeaderLinks />
          </Navbar.Container>
        </Navbar.Menu>
      </Navbar>
    </>
  );
};

export default Header;
