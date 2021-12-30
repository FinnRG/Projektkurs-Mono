import ReactHlsPlayer from 'react-hls-player';
import { useParams } from 'react-router';

const Player = () => {

    const params = useParams();

    return <ReactHlsPlayer
        src={'http://localhost:8000/get/' + params.video_id}
        autoPlay={false}
        controls={true}
        className='is-flex is-justify-content-center'
        style={{ display: 'block', width: '60%', margin: '0 auto' }}
        height='auto' />
}

export default Player;