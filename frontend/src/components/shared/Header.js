import { Box, Breadcrumb } from 'react-bulma-components';
import { Link } from 'react-router-dom';
import { client } from '../../App';

const Header = () => {

    const handleLogout = () => {
        client
            .post('http://localhost:8000/user/logout')
            .then((resp) => console.log('Logout successsfull!'))
            .catch((err) => console.log('Unable to logout!'))
    }

    return <>
        <Box>
            <Breadcrumb>
                <Breadcrumb.Item>
                    <Link to='player'>
                        Player
                    </Link>
                    <Link to='videos'>
                        Videos
                    </Link>
                    <Link to='upload'>
                        Upload
                    </Link>
                    <Link to='login'>
                        Login
                    </Link>
                    <Link to='register'>
                        Register
                    </Link>
                    <Link to='id'>
                        Id
                    </Link>
                    <Link to='player' onClick={(e) => handleLogout(e)} >
                        Logout
                    </Link>
                </Breadcrumb.Item>
            </Breadcrumb>
        </Box></>
}

export default Header;