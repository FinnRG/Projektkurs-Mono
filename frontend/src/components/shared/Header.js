import { Box, Level } from 'react-bulma-components';
import { Link } from 'react-router-dom';
import { client } from '../../App';

const { Side, Item } = Level;

const Header = () => {

    const handleLogout = () => {
        client
            .post('http://localhost:8000/user/logout')
            .then((resp) => console.log('Logout successsfull!'))
            .catch((err) => console.log('Unable to logout!'))
    }

    return <>
        <Box>
            <Level>
                <Side>
                    <Item>
                        <Link to='player'>
                            Player
                        </Link>
                    </Item>
                    <Item>
                        <Link to='videos'>
                            Videos
                        </Link>
                    </Item>
                    <Item>
                        <Link to='upload'>
                            Upload
                        </Link>
                    </Item>
                    <Item>
                        <Link to='login'>
                            Login
                        </Link>
                    </Item>
                    <Item>
                        <Link to='register'>
                            Register
                        </Link>
                    </Item>
                    <Item>
                        <Link to='id'>
                            Id
                        </Link>
                    </Item>
                    <Item>
                        <Link to='player' onClick={(e) => handleLogout(e)} >
                            Logout
                        </Link>
                    </Item>
                </Side>
            </Level>
        </Box></>
}

export default Header;