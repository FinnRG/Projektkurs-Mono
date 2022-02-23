import { useEffect, useState } from 'react';
import PlaylistLink from '../components/list/PlaylistLink';
import client from '../global/client';

const Playlist = () => {
  const [playlists, setPlaylists] = useState([]);

  useEffect(() => {
    client({
      method: 'GET',
      url: '/playlist/get',
    }).then((resp) => setPlaylists(resp.data));
  }, []);

  return (
    <>
      {playlists.length == 0 && (<p>You can create a playlist in the extended view panel </p>)}
      {playlists.map((playlist, index) => (
        <PlaylistLink id={playlist.id} title={playlist.title} key={index} />
      ))}
    </>
  );
};

export default Playlist;
