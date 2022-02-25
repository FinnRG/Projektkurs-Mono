import React, { useEffect, useState } from "react";
import { Navbar } from "react-bulma-components";
import { useLocation } from "react-router";
import useOnClickOutside from 'use-onclickoutside';

const HeaderDropdown = (props) => {

    const [active, setActive] = useState(false);
    const ref = React.useRef(null);
    useOnClickOutside(ref, () => setActive(false));

    const location = useLocation();

    useEffect(() => {
        setActive(false);
    }, [location]);

    return <div ref={ref} className="navbar-item p-0 m-0">
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