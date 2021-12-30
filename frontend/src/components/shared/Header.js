import { Box, Breadcrumb } from 'react-bulma-components';
import { Link } from 'react-router-dom';

const Header = () => {
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
                </Breadcrumb.Item>
            </Breadcrumb>
        </Box></>
}

export default Header;