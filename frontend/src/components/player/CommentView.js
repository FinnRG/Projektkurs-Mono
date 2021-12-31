import { useEffect, useState } from 'react';
import { Box, Section } from 'react-bulma-components';
import { useParams } from 'react-router';
import { client } from '../../App';
import Comment from '../comment/Comment';
import CreateComment from '../comment/CreateComment';

const CommentView = () => {

    const params = useParams();

    const [comments, setComments] = useState([]);

    const getComments = () => {
        client.get('http://localhost:8000/comment/get', {
            params: {
                video_id: params.video_id
            }
        })
            .then((resp) => setComments(resp.data))
            .catch((err) => console.log(err));
    };

    const onCommentCreation = () => {
        console.log(comments);
        getComments();
        console.log(comments);
    }

    useEffect(() => {
        getComments();
    }, []);

    return <Section>
        <Box>
            {comments.map((comment) => <Comment username={comment.name} content={comment.content} />)}
            <CreateComment onCommentCreation={onCommentCreation} />
        </Box>
    </Section>
}

export default CommentView;