import { autocomplete } from '@algolia/autocomplete-js';
import { useEffect, useState } from 'react';
import { Button, Form, Panel } from 'react-bulma-components';
import { useParams } from 'react-router';
import client from '../../global/client';

const { Block, Header, Tabs } = Panel;

const ExtendedViewPanel = (props) => {
  const params = useParams();

  const [currentTab, setCurrentTab] = useState('playlists');

  const [tags, setTags] = useState([]);
  const [playlists, setPlaylists] = useState([]);
  const [playlistTitle, setPlaylistTitle] = useState("");

  const setPlaylistAutocomplete = () => {

    const node = document.getElementById('autocomplete-playlist');

    while (node.firstChild) {
      node.innerHTML = '';
    }

    client.get('/playlist/get').then((resp) => {
      setPlaylists(resp.data);
      autocomplete({
        container: '#autocomplete-playlist',
        placeholder: 'Search for playlists',
        getSources({ setQuery, setIsOpen }) {
          return [
            {
              sourceId: 'id',
              getItems({ query }) {
                return resp.data.filter(
                  ({ title }) =>
                    title.toLowerCase().includes(query.toLowerCase())
                );
              },
              onSelect({ item }) {
                setQuery('');
                setIsOpen(false);
                addVideo(item)
              },
              getItemUrl({ item }) {
                return item.id;
              },
              templates: {
                item({ item }) {
                  return `${item.title}`;
                },
              },
            },
          ];
        },
        // Completely disables the built in navigation
        navigator: {
          navigate() { },
          navigateNewTab() { },
          navigateNewWindow() { },
        },
      });
    });
  }

  useEffect(() => {
    client.get('/tag/get').then((resp) => {
      setTags(resp.data);
      autocomplete({
        container: '#autocomplete-tag',
        placeholder: 'Search for tags',
        getSources({ setQuery, setIsOpen }) {
          return [
            {
              sourceId: 'id',
              getItems({ query }) {
                return resp.data.filter(
                  ({ deleted, name }) =>
                    deleted !== true &&
                    name.toLowerCase().includes(query.toLowerCase())
                );
              },
              onSelect({ item }) {
                setQuery('');
                setIsOpen(false);
                addTag(item.id);
              },
              getItemUrl({ item }) {
                return item.id;
              },
              templates: {
                item({ item }) {
                  return `${item.name}`;
                },
              },
            },
          ];
        },
        // Completely disables the built in navigation
        navigator: {
          navigate() { },
          navigateNewTab() { },
          navigateNewWindow() { },
        },
      });
    });

    setPlaylistAutocomplete();
  }, []);

  const addTag = (tagId) => {
    client({
      method: 'POST',
      url: '/tag/add',
      params: {
        video_id: params.video_id,
        tag_id: tagId,
      },
    }).then(() => props.update());
  };

  const addVideo = (playlist) => {
    client({
      method: 'POST',
      url: '/playlist/add',
      params: {
        playlist_id: playlist.id,
        video_id: params.video_id,
      }
    })
  }

  const createPlaylist = () => {
    client({
      method: 'POST',
      url: '/playlist/create',
      params: {
        title: playlistTitle
      }
    }).then(() => setPlaylistAutocomplete());
  }

  return (
    <Panel color='info'>
      <Header>Extended View</Header>

      <Tabs>
        <Tabs.Tab onClick={() => setCurrentTab('tag')}>Add tag</Tabs.Tab>
        <Tabs.Tab onClick={() => setCurrentTab('playlists')}>
          Playlists
        </Tabs.Tab>
      </Tabs>

      <Block display={currentTab === 'playlists' ? 'visible' : 'hidden'}>
        <form onSubmit={(e) => e.preventDefault()}>
          <Form.Field className='has-addons mb-2'>
            <Button onClick={(e) => {
              createPlaylist(e);
              e.preventDefault();
            }} color='info'>+</Button>
            <Form.Input placeholder='Playlist Name' value={playlistTitle} onChange={(e) => setPlaylistTitle(e.target.value)}></Form.Input>
          </Form.Field>
          <Form.Field className='has-addons'>
            <div id='autocomplete-playlist'></div>
          </Form.Field>
        </form>
      </Block>

      <Block display={currentTab === 'tag' ? 'visible' : 'hidden'}>
        <form>
          <Form.Field className='has-addons'>
            <div id='autocomplete-tag'></div>
          </Form.Field>
        </form>
      </Block>
    </Panel>
  );
};

export default ExtendedViewPanel;
