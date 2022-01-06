import React from 'react';
import ReactHlsPlayer from 'react-hls-player';

const StaticPlayer = (props) => {
    return <ReactHlsPlayer
        src={'http://localhost:8000/get/' + props.video_id}
        autoPlay={false}
        controls={true}
        className='mr-6'
        height='auto' />
}

// Prevent unnecessary player rerendering (which cause a player resize and disturbs the UX)
export default React.memo(StaticPlayer);