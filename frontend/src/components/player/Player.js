import { faBars } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useState } from 'react';
import { Box, Button, Columns, Container, Level } from 'react-bulma-components';
import { useParams } from 'react-router';
import Rating from './Rating';
import StaticPlayer from './StaticPlayer';
import ExtendedView from './ExtendedView';

const Player = () => {

    const params = useParams();
    const [extendedView, setExtendedView] = useState(false);

    const handleExtendedView = () => {
        setExtendedView(!extendedView);
    }

    return <Box>
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
                <Button onClick={() => handleExtendedView()} >
                    <FontAwesomeIcon icon={faBars} />
                </Button>
            </Button.Group>
        </Container>
    </Box>
}

export default Player;