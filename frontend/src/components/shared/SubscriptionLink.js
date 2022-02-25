import { useEffect, useState } from "react"
import { Navbar } from "react-bulma-components"
import { Link } from "react-router-dom";
import client from "../../global/client";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faBell, faBellSlash } from '@fortawesome/free-solid-svg-icons';

const SubscriptionLink = () => {

    const [notified, setNotified] = useState(false);

    useEffect(() => {
        client.get('/user/notifications')
            .then((resp) => {
                if (resp.data.length > 0) {
                    setNotified(true);
                }
            })
    }, []);

    return <Navbar.Item renderAs={Link} to='notifications'>
        <FontAwesomeIcon
            icon={notified ? faBell : faBellSlash}
            color={notified ? 'red' : 'grey'}
        />
    </Navbar.Item>
}

export default SubscriptionLink;