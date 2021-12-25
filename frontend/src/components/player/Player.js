import { useState } from 'react';
import { Form } from 'react-bulma-components';
import ReactHlsPlayer from 'react-hls-player';

const { Input } = Form;


const Player = () => {
    const [hlsUrl, setHlsUrl] = useState(
        'http://localhost:8000/get/'
    );

    return (<main>
        <Input type='text'
            placeholder='HLS Url...'
            value={hlsUrl}
            onChange={(e) => setHlsUrl(e.target.value)} />
        <ReactHlsPlayer
            src={hlsUrl}
            autoPlay={false}
            controls={true}
            width='60%'
            height='auto' />
    </main>
    )
}
export default Player;