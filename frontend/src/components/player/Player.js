import { faBars, faPen } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useContext, useEffect, useState } from 'react';
import { Box, Button, Columns, Container, Level } from 'react-bulma-components';
import { Navigate, useParams } from 'react-router';
import userContext from '../../global/userContext';
import ExtendedView from './ExtendedView';
import Rating from './Rating';
import StaticPlayer from './StaticPlayer';

const Player = () => {
  const params = useParams();
  const user = useContext(userContext);
  const [extendedView, setExtendedView] = useState(false);
  const [redirect, setRedirect] = useState(false);

  const handleExtendedView = () => {
    setExtendedView(!extendedView);
  };

  useEffect(() => {
    localStorage.setItem('last_video_id', params.video_id);
  }, []);

  if (redirect) {
    return <Navigate to={`/video/edit/${params.video_id}`} />;
  }

  return (
    <Box>
      <Container>
        <Level>
          <Columns.Column size={extendedView ? 9 : 12}>
            <StaticPlayer video_id={params.video_id} />
          </Columns.Column>
          <Columns.Column size={extendedView ? 3 : 0}>
            {extendedView && <ExtendedView />}
          </Columns.Column>
        </Level>
        <Button.Group>
          <Rating />
          {user.loggedIn && (
            <Button onClick={() => setRedirect(true)}>
              <FontAwesomeIcon icon={faPen} />
            </Button>
          )}
          <Button onClick={() => handleExtendedView()}>
            <FontAwesomeIcon icon={faBars} />
          </Button>
        </Button.Group>
      </Container>
    </Box>
  );
};

export default Player;
