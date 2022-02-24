import { Navbar } from 'react-bulma-components';
import { Link } from 'react-router-dom';

const HeaderLink = (props) => {
  return (
    <Navbar.Item
      onClick={props.onClick}
      renderAs={Link}
      to={props.to ? props.to : props.text.toLowerCase()}
    >
      {props.text}
    </Navbar.Item>
  );
};

export default HeaderLink;
