import { useEffect, useState } from "react";
import { Navbar } from "react-bulma-components";
import { useLocation } from "react-router";

const HeaderDropdown = (props) => {

    const [active, setActive] = useState(false);

    const location = useLocation();

    useEffect(() => {
        setActive(false);
    }, [location]);

    return <div className="navbar-item p-0 m-0" tagIndex={0} onBlur={() => setActive(false)}>
        <Navbar.Item className={active ? 'is-active' : ''}>
            <Navbar.Link onClick={() => setActive(!active)}>
                {props.text}
            </Navbar.Link>
            <Navbar.Dropdown className={active ? '' : 'is-hidden'}>
                {props.children}
            </Navbar.Dropdown>
        </Navbar.Item>
    </div>
}

export default HeaderDropdown;