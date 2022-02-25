import { useEffect, useState } from "react"
import { Card, Columns } from "react-bulma-components"
import { Link } from "react-router-dom"
import client from "../../global/client"

const Notification = ({ tagId, videoId }) => {

    const [tagName, setTagName] = useState('');

    useEffect(() => {
        client.get('/tag/get', {
            params: {
                tag_id: tagId
            },
            withCredentials: false,
        }).then((resp) => setTagName(resp.data.name))
    }, [tagId])

    return <Card>
        <Card.Content>
            <Columns>
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