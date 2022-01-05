import { Block, Level, Panel } from 'react-bulma-components';
import { useParams } from 'react-router';
import { client } from '../../App';
import Trash from '../shared/Trash';

const Tag = (props) => {

    const params = useParams();

    const onDelete = () => {
        client({
            method: 'post',
            params: {
                tag_id: props.tag_id,
                video_id: params.video_id,
            },
            url: 'http://localhost:8000/tag/remove',
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
        })
            .then(() => props.onSuccess())
            .catch((err) => console.log(err));
    }

    return <Block >
        <Level justifyContent='space-between'>
            {props.name}
            <Panel.Icon>
                <Trash onClick={() => onDelete()} />
            </Panel.Icon>
        </Level>
    </Block>
}

export default Tag;