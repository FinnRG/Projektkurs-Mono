import { useEffect, useState } from 'react';
import { Box, Section } from 'react-bulma-components';
import { useParams } from 'react-router';
import client from '../../global/client';
import Comment from '../comment/Comment';
import CreateComment from '../comment/CreateComment';

const CommentView = () => {
  const params = useParams();

  const [comments, setComments] = useState([]);

  const getComments = () => {
    client
      .get('/comment/get', {
        params: {
          video_id: params.video_id,
        },
      })
      .then((resp) => setComments(resp.data))
      .catch((err) => console.log(err));
  };

  useEffect(() => {
    getComments();
  }, []);

  return (
    <Section>
      <Box>
        {comments.map((comment, index) => (
          <Comment
            key={index}
            username={comment.name}
            content={comment.content}
            comment_id={comment.id}
            onUpdate={getComments}
          />
        ))}
        <CreateComment onUpdate={getComments} />
      </Box>
    </Section>
  );
};

export default CommentView;
