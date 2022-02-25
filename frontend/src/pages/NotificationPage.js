import { useEffect, useState } from "react"
import Notification from "../components/notifications/Notification";
import client from "../global/client"

const NotificationPage = () => {

    const [notifications, setNotifications] = useState([]);

    useEffect(() => {
        client.get('/user/notifications')
            .then((resp) => {
                setNotifications(resp.data);
            });
    }, []);

    return <>
        {notifications.map((notification, index) => (
            <Notification key={index} tagId={notification.tag_id} videoId={notification.video_id} />
        ))}
    </>
}

export default NotificationPage