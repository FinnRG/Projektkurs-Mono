import { useState } from 'react';
import { Button, Form, Media } from 'react-bulma-components';
import { useParams } from 'react-router';
import client from '../../global/client';

const { Control, Field, Textarea } = Form;

const CreateComment = (props) => {
  const params = useParams();

  const [content, setContent] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSubmit = (e) => {
    setLoading(true);
    client
      .post(
        '/comment/create',
        {},
        {
          params: {
            video_id: params.video_id,
            content: content,
          },
        }
      )
      .then(() => {
        setLoading(false);
        setContent('');
        props.onUpdate();
      })
      .catch((err) => console.log(err));
  };

  return (
    <Media>
      <Media.Item align='center'>
        <Field>
          <Control loading={loading}>
            <Textarea
              onChange={(e) => setContent(e.target.value)}
              value={content}
              size='medium'
              placeholder='Enter your comment'
            />
          </Control>
        </Field>
        <Field>
          <Control>
            <Button onClick={(e) => handleSubmit(e)}>Post comment</Button>
          </Control>
        </Field>
      </Media.Item>
    </Media>
  );
};

export default CreateComment;
