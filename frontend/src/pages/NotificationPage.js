import { useEffect, useState } from "react"
import Notification from "../components/notifications/Notification";
import client from "../global/client"

const NotificationPage = () => {

    const [notifications, setNotifications] = useState([]);

    useEffect(() => {
        client.get('/user/notifications/list')
            .then((resp) => {
                setNotifications(resp.data);
            });
    }, []);

    const onDelete = (tagId) => {
        setNotifications(notifications.filter(({ id }) => id !== tagId))
    }

    return <>
        {notifications.map((notification, index) => (
            <Notification key={index} onDelete={onDelete} tagId={notification.tag_id} videoId={notification.video_id} />
        ))}
    </>
}

export default NotificationPage