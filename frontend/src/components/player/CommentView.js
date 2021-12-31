import { useEffect, useState } from 'react';
import { Box, Section } from 'react-bulma-components';
import { useParams } from 'react-router';
import { client } from '../../App';
import Comment from '../comment/Comment';
import CreateComment from '../comment/CreateComment';

const CommentView = (props) => {

    const params = useParams();

    const [comments, setComments] = useState([]);


    useEffect(() => {
        const getComments = (video_id) => {
            client.get('http://localhost:8000/comment/get', {
                params: {
                    video_id: video_id
                }
            })
                .then((resp) => setComments(resp.data))
                .catch((err) => console.log(err));
        };

        getComments(params.video_id);
    }, [params.video_id]);

    return <Section>
        <Box>
            {comments.map((comment) => <Comment username={comment.user_id} content={comment.content} />)}
            <CreateComment />
        </Box>
    </Section>
}

export default CommentView;