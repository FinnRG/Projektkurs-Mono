import { autocomplete } from '@algolia/autocomplete-js';
import { useEffect, useState } from 'react';
import { Form, Panel } from 'react-bulma-components';
import { useParams } from 'react-router';
import client from '../../global/client';

const { Block, Header, Tabs } = Panel;

const ExtendedViewPanel = (props) => {

    const params = useParams();

    const [currentTab, setCurrentTab] = useState('playlists');

    const [tags, setTags] = useState([]);

    useEffect(() => {
        client.get('/tag/get')
            .then((resp) => {
                setTags(resp.data);
                autocomplete({
                    container: '#autocomplete',
                    placeholder: 'Search for tags',
                    getSources({ setQuery, setIsOpen }) {
                        return [{
                            sourceId: 'id',
                            getItems({ query }) {
                                return resp.data.filter(({ deleted, name }) => deleted !== true && name.toLowerCase().includes(query.toLowerCase()))
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
                                    return `${item.name}`
                                }
                            }
                        }];
                    },
                    // Completely disables the built in navigation
                    navigator: {
                        navigate() { },
                        navigateNewTab() { },
                        navigateNewWindow() { },
                    }
                });
            });
    }, []);

    const addTag = (tagId) => {
        client({
            method: 'POST',
            url: '/tag/add',
            params: {
                video_id: params.video_id,
                tag_id: tagId,
            }
        })
            .then(() => props.update());
    }

    return <Panel color='info'>
        <Header>
            Extended View
        </Header>

        <Tabs>
            <Tabs.Tab onClick={() => setCurrentTab('tag')}>
                Add tag
            </Tabs.Tab>
            <Tabs.Tab onClick={() => setCurrentTab('playlists')}>
                Playlists
            </Tabs.Tab>
        </Tabs>

        <Block display={currentTab === 'tag' ? 'visible' : 'hidden'}>
            <Form.Field className='has-addons'>
                <div id='autocomplete'></div>
            </Form.Field>
        </Block>

    </Panel>
}

export default ExtendedViewPanel;