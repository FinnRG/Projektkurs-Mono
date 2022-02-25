import { useEffect, useState } from "react"
import { Card, Columns } from "react-bulma-components"
import { Link } from "react-router-dom"
import client from "../../global/client"
import Trash from "../shared/Trash"

const Notification = ({ tagId, videoId, onDelete }) => {

    const [tagName, setTagName] = useState('');

    useEffect(() => {
        client.get('/tag/get', {
            params: {
                tag_id: tagId
            },
            withCredentials: false,
        }).then((resp) => setTagName(resp.data.name))
    }, [tagId])

    const remove = () => {
        client.post('/user/notifications/remove', {}, {
            params: {
                tag_id: tagId,
                video_id: videoId,
            }
        }).then(() => onDelete())
    }

    return <Card>
        <Card.Content>
            <Columns>
                <Columns.Column className='is-narrow'>
                    <Trash onClick={() => remove()} />
                </Columns.Column>
                <Columns.Column>
                    <Link to={'/player/' + videoId}>
                        New video tagged with: {tagName}
                    </Link>
                </Columns.Column>
            </Columns>
        </Card.Content>
    </Card>
}

export default Notification;