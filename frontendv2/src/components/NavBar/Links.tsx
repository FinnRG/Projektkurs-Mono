import { createStyles, Tooltip, UnstyledButton } from '@mantine/core';
import { useState } from 'react';
import {
  Icon as TablerIcon,
  Logout,
  Video,
  PlayerPlay,
  Tags,
  Playlist,
} from 'tabler-icons-react';
import { useNavigate } from 'react-router-dom';

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
  route: string;
  onClick?(): void;
}

function NavbarLink({ icon: Icon, label, active, onClick }: NavbarLinkProps) {
  const { classes, cx } = useStyles();
  return (
    <Tooltip label={label} position='right' withArrow transitionDuration={0}>
      <UnstyledButton onClick={onClick} className={cx(classes.link)}>
        <Icon />
      </UnstyledButton>
    </Tooltip>
  );
}

const linkData = [
  { icon: PlayerPlay, label: 'Player', route: '/player' },
  { icon: Video, label: 'Videos', route: '/videos' },
  { icon: Playlist, label: 'Playlists', route: '/playlists' },
  { icon: Tags, label: 'Tags', route: '/tags' },
];

const Links = () => {
  const [active, setActive] = useState(2);
  const navigate = useNavigate();
  const links = linkData.map((link, index) => (
    <NavbarLink
      {...link}
      key={link.label}
      active={index === active}
      onClick={() => {
        setActive(index);
        navigate(link.route, { replace: true });
      }}
    />
  ));
  return <>{links}</>;
};

export { Links, NavbarLink };
