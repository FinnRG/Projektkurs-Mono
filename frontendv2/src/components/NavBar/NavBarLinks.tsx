import { createStyles, Tooltip, UnstyledButton } from '@mantine/core';
import { useContext, useState } from 'react';
import { Icon as TablerIcon, Video, Upload } from 'tabler-icons-react';
import { useNavigate } from 'react-router-dom';
import UserContext from '../../context/userContext';

const useStyles = createStyles((theme) => ({
  link: {
    width: 50,
    height: 50,
    borderRadius: theme.radius.md,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    color:
      theme.colorScheme === 'dark'
        ? theme.colors.dark[0]
        : theme.colors.gray[7],

    '&:hover': {
      backgroundColor:
        theme.colorScheme === 'dark'
          ? theme.colors.dark[5]
          : theme.colors.gray[0],
    },
  },

  active: {
    '&, &:hover': {
      backgroundColor:
        theme.colorScheme === 'dark'
          ? theme.fn.rgba(theme.colors[theme.primaryColor][9], 0.25)
          : theme.colors[theme.primaryColor][0],
      color:
        theme.colors[theme.primaryColor][theme.colorScheme === 'dark' ? 4 : 7],
    },
  },
}));

interface NavbarLinkProps {
  icon: TablerIcon;
  label: string;
  active?: boolean;
  route?: string;
  restricted?: boolean;
  onClick?(): void;
}

const NavbarLink = ({ icon: Icon, label, onClick, route }: NavbarLinkProps) => {
  const { classes, cx } = useStyles();
  const navigate = useNavigate();
  return (
    <Tooltip label={label} position='right' withArrow transitionDuration={0}>
      <UnstyledButton
        onClick={onClick ? onClick : () => navigate(route || '')}
        className={cx(classes.link)}
      >
        <Icon />
      </UnstyledButton>
    </Tooltip>
  );
};

const linkData = [
  { icon: Video, label: 'Videos', route: '/videos' },
  { icon: Upload, label: 'Upload', route: '/create', restricted: true },
];

const NavBarLinks = () => {
  const [active, setActive] = useState(2);
  const navigate = useNavigate();
  const { user } = useContext(UserContext);
  const links = linkData
    .filter((link) => {
      if (user == null) {
        return !(link.restricted != null && link.restricted);
      }
      return true;
    })
    .map((link, index) => (
      <NavbarLink
        {...link}
        key={link.label}
        active={index === active}
        onClick={() => {
          setActive(index);
          navigate(link.route);
        }}
      />
    ));
  return <>{links}</>;
};

export { NavBarLinks as Links, NavbarLink };
