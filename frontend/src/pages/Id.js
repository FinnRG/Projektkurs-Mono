import { useState } from 'react';
import { client } from '../App';

const Id = () => {
    const [id, setId] = useState('');
    client.get('http://localhost:8000/user/id', {
        headers: {
            'Accept': 'text/html',
            'Content-Type': 'text/plain',
        }
    })
        .then((resp) => setId(resp.data))

    return <>
        <p>{id}</p>
    </>
}

export default Id;