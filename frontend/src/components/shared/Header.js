import { useEffect, useState } from 'react';
import { Box, Navbar } from 'react-bulma-components';
import HeaderLinks from './HeaderLinks';
import { useLocation } from 'react-router';
import HeaderLink from './HeaderLink';
import SubscriptionLink from './SubscriptionLink';

const Header = () => {
  const [showMenu, setShowMenu] = useState(false);

  const location = useLocation();

  useEffect(() => {
    setShowMenu(false);
  }, [location]);

  return (
    <>
      <Navbar mb={showMenu ? 4 : 0} transparent={true}>
        <Navbar.Brand>
          <Navbar.Burger
            className={showMenu ? 'is-active' : ''}
            onClick={() => setShowMenu(!showMenu)}
          />
        </Navbar.Brand>

        <Navbar.Menu className={showMenu ? 'is-active' : ''}>
          <Navbar.Container>
            <HeaderLinks />
          </Navbar.Container>
          <Navbar.Container align='end'>
            <SubscriptionLink />
          </Navbar.Container>
        </Navbar.Menu>
      </Navbar>
    </>
  );
};

export default Header;
