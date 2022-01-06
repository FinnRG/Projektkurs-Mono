import { useState } from 'react';
import { Button, Form, Panel } from 'react-bulma-components';
import { useParams } from 'react-router';
import { client } from '../../App';

const { Block, Header, Tabs } = Panel;

const ExtendedViewPanel = (props) => {

    const params = useParams();

    const [currentTab, setCurrentTab] = useState('playlists');
    const [tagToAdd, setTagToAdd] = useState('');

    const addTag = () => {
        client({
            method: 'POST',
            url: 'http://localhost:8000/tag/add',
            params: {
                video_id: params.video_id,
                tag_id: tagToAdd,
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

        {currentTab === 'tag' && (
            <Block>
                <Form.Field className='has-addons'>
                    <Form.Input onChange={(e) => setTagToAdd(e.target.value)} />
                    <Button onClick={() => addTag()} >
                        Add
                    </Button>
                </Form.Field>
            </Block>
        )}

    </Panel>
}

export default ExtendedViewPanel;