import {
  ActionIcon,
  AppShell,
  Burger,
  Group,
  Header,
  MediaQuery,
  Navbar,
  useMantineColorScheme,
  useMantineTheme,
} from '@mantine/core';
import { useViewportSize } from '@mantine/hooks';
import { useContext, useState } from 'react';
import { Outlet } from 'react-router-dom';
import { Login, Logout, MoonStars, Sun } from 'tabler-icons-react';
import { Links, NavbarLink } from './components/NavBar/NavBarLinks';
import UserContext, { logout } from './context/userContext';

function Context() {
  const [opened, setOpened] = useState(false);
  const { user } = useContext(UserContext);
  const theme = useMantineTheme();
  const { colorScheme, toggleColorScheme } = useMantineColorScheme();

  return (
    <AppShell
      // navbarOffsetBreakpoint controls when navbar should no longer be offset with padding-left
      navbarOffsetBreakpoint='sm'
      // fixed prop on AppShell will be automatically added to Header and Navbar
      fixed
      navbar={
        <Navbar
          p='md'
          // Breakpoint at which navbar will be hidden if hidden prop is true
          hiddenBreakpoint='sm'
          // Hides navbar when viewport size is less than value specified in hiddenBreakpoint
          hidden={!opened}
          // when viewport size is less than theme.breakpoints.sm navbar width is 100%
          // viewport size > theme.breakpoints.sm – width is 300px
          // viewport size > theme.breakpoints.lg – width is 400px
          width={opened ? { base: '100%' } : { base: 80 }}
        >
          <Navbar.Section grow>
            <Group direction='column' align='center' spacing={0}>
              <Links />
            </Group>
          </Navbar.Section>
          <Navbar.Section>
            <Group direction='column' align='center' spacing={0}>
              <ActionIcon
                onClick={() => toggleColorScheme()}
                size='lg'
                mb={3}
                sx={(theme) => ({
                  backgroundColor:
                    theme.colorScheme === 'dark'
                      ? theme.colors.dark[6]
                      : theme.colors.gray[0],
                  color:
                    theme.colorScheme === 'dark'
                      ? theme.colors.yellow[4]
                      : theme.colors.blue[6],
                })}
              >
                {colorScheme === 'dark' ? (
                  <Sun size={18} />
                ) : (
                  <MoonStars size={18} />
                )}
              </ActionIcon>
              {!user && <NavbarLink icon={Login} label='Login' route='login' />}
              {user && (
                <NavbarLink icon={Logout} label='Logout' onClick={logout} />
              )}
            </Group>
          </Navbar.Section>
        </Navbar>
      }
      header={
        <Header
          height={useViewportSize().width > 800 ? 0 : 70}
          hidden={useViewportSize().width > 800 ? true : false}
          p='md'
        >
          <MediaQuery largerThan='sm' styles={{ display: 'none' }}>
            <Burger
              opened={opened}
              onClick={() => setOpened(!opened)}
              size='sm'
              color={theme.colors.gray[6]}
              mr='xl'
            />
          </MediaQuery>
        </Header>
      }
    >
      <Outlet />
    </AppShell>
  );
}

export default Context;
