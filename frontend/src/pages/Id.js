import { useContext } from 'react';
import userContext from '../global/userContext';

const Id = () => {
    const user = useContext(userContext);

    return <>
        <p>{user.userId}</p>
    </>
}

export default Id;