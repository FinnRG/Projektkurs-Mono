import React, { useRef } from 'react';
import {
  Text,
  Group,
  Button,
  createStyles,
  MantineTheme,
  useMantineTheme,
  Progress,
} from '@mantine/core';
import { Dropzone, DropzoneStatus } from '@mantine/dropzone';
import { CloudUpload } from 'tabler-icons-react';

const useStyles = createStyles((theme) => ({
  wrapper: {
    position: 'relative',
    marginBottom: 30,
  },

  dropzone: {
    borderWidth: 1,
    paddingBottom: 50,
  },

  icon: {
    color:
      theme.colorScheme === 'dark'
        ? theme.colors.dark[3]
        : theme.colors.gray[4],
  },

  control: {
    position: 'absolute',
    width: 250,
    left: 'calc(50% - 125px)',
    bottom: -20,
    transition: 'background-color 150ms ease',
  },

  progress: {
    position: 'absolute',
    bottom: -1,
    right: -1,
    left: -1,
    top: -1,
    height: 'auto',
    backgroundColor: 'transparent',
    zIndex: 0,
  },
  label: {
    position: 'relative',
    zIndex: 1,
  },
}));

const getActiveColor = (status: DropzoneStatus, theme: MantineTheme) => {
  return status.accepted
    ? theme.colors[theme.primaryColor][6]
    : status.rejected
    ? theme.colors.red[6]
    : theme.colorScheme === 'dark'
    ? theme.colors.dark[0]
    : theme.black;
};

interface DropzoneWithButtonProps {
  onDrop: (arg0: File[]) => void;
  progress: number;
  loaded: boolean;
}

const DropzoneWithButton = ({
  progress,
  onDrop,
  loaded,
}: DropzoneWithButtonProps) => {
  const theme = useMantineTheme();
  const { classes } = useStyles();
  const openRef = useRef<() => void>();

  return (
    <div className={classes.wrapper}>
      <Dropzone
        openRef={openRef}
        onDrop={onDrop}
        className={classes.dropzone}
        radius='md'
      >
        {(status) => (
          <div style={{ pointerEvents: 'none' }}>
            <Group position='center'>
              <CloudUpload size={50} color={getActiveColor(status, theme)} />
            </Group>
            <Text
              align='center'
              weight={700}
              size='lg'
              mt='xl'
              sx={{ color: getActiveColor(status, theme) }}
            >
              {'Upload video'}
            </Text>
            <Text align='center' size='sm' mt='xs' color='dimmed'>
              Drag&apos;n&apos;drop files here to upload.
            </Text>
          </div>
        )}
      </Dropzone>

      <Button
        className={classes.control}
        size='md'
        radius='xl'
        onClick={() => openRef.current()}
        fullWidth
      >
        <div className={classes.label}>
          {progress !== 0
            ? 'Uploading files'
            : loaded
            ? 'Files uploaded'
            : 'Upload files'}
        </div>
        {progress !== 0 && (
          <Progress
            value={progress}
            className={classes.progress}
            color={theme.fn.rgba(theme.colors[theme.primaryColor][2], 0.35)}
            radius='sm'
          />
        )}
      </Button>
    </div>
  );
};

export default DropzoneWithButton;
