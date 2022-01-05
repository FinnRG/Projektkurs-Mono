import { Box, Container, Level } from 'react-bulma-components';
import ReactHlsPlayer from 'react-hls-player';
import { useParams } from 'react-router';
import Rating from './Rating';
import TagView from './TagView';

const Player = () => {

    const params = useParams();

    return <Box>
        <Container>
            <Level>
                <ReactHlsPlayer
                    src={'http://localhost:8000/get/' + params.video_id}
                    autoPlay={false}
                    controls={true}
                    className='mr-6'
                    height='auto' />
                <TagView />
            </Level>
            <Rating />
        </Container>
    </Box>
}

export default Player;