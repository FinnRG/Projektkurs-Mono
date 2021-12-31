import { Box, Container } from 'react-bulma-components';
import ReactHlsPlayer from 'react-hls-player';
import { useParams } from 'react-router';
import Rating from './Rating';

const Player = () => {

    const params = useParams();

    return <Box>
        <Container>
            <ReactHlsPlayer
                src={'http://localhost:8000/get/' + params.video_id}
                autoPlay={false}
                controls={true}
                height='auto' />
            <Rating />
        </Container>
    </Box>
}

export default Player;