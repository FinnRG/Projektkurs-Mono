import { Box, Media, Section, Content } from 'react-bulma-components';

const { Item } = Media;

const Comment = (props) => {
    return <Media renderAs='article'>
        <Item align='center'>
            <Content>
                <p>
                    <strong>{props.username}</strong>
                    <br />
                    {props.content}
                    <br />
                    {/* TODO: Add like functionality here */}
                </p>
            </Content>
        </Item>
    </Media>
}

export default Comment;