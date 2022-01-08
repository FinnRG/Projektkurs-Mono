import { useContext } from 'react';
import { Box, Level } from 'react-bulma-components';
import { Link } from 'react-router-dom';
import client from '../../global/client';
import userContext from '../../global/userContext';

const { Side, Item } = Level;

const Header = () => {

    const user = useContext(userContext);

    const handleLogout = () => {
        client
            .post('http://localhost:8000/user/logout')
            .then(() => {
                user.setUserId(null);
                user.setLoggedIn(false);
            })
            .catch((err) => {
                console.log(err);
            })
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
                    {user.loggedIn && (
                        <>
                            <Item>
                                <Link to='upload'>
                                    Upload
                                </Link>
                            </Item>
                            <Item>
                                <Link to='tag/edit'>
                                    Edit tags
                                </Link>
                            </Item>
                        </>
                    )}
                    {!user.loggedIn && (
                        <>
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
                        </>
                    )}
                    {user.loggedIn && (
                        <>
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
                        </>
                    )}
                </Side>
            </Level>
        </Box></>
}

export default Header;