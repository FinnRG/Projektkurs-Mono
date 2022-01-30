import { faThumbsDown, faThumbsUp } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useEffect, useState } from 'react';
import { Button, Icon } from 'react-bulma-components';
import { useParams } from 'react-router';
import client from '../../global/client';

const Rating = () => {
  const params = useParams();

  const [ratingData, setRatingData] = useState({});

  const axiosParams = {
    params: {
      video_id: params.video_id,
    },
  };

  useEffect(() => {
    const getLikeData = () => {
      client
        .get('/like/info', axiosParams)
        .then((resp) => setRatingData(resp.data));
    };
    getLikeData();
  }, []);

  const handleLike = (value) => {
    client
      .post(
        '/like/add',
        {},
        {
          params: {
            video_id: params.video_id,
            value,
          },
        }
      )
      .then(() => {
        let updatedData = Object.assign({}, ratingData);
        if (ratingData.user_like != null && ratingData.user_like !== value) {
          value ? (updatedData.dislikes -= 1) : (updatedData.likes -= 1);
        }
        updatedData.user_like = value;
        value ? (updatedData.likes += 1) : (updatedData.dislikes += 1);
        setRatingData(updatedData);
      });
  };

  const handleLikeRemove = () => {
    client.post('/like/remove', {}, axiosParams).then(() => {
      let updatedData = Object.assign({}, ratingData);
      ratingData.user_like
        ? (updatedData.likes -= 1)
        : (updatedData.dislikes -= 1);
      updatedData.user_like = null;
      setRatingData(updatedData);
    });
  };

  return (
    <>
      <Button
        onClick={() => {
          if (ratingData.user_like == true) {
            handleLikeRemove();
          } else {
            handleLike(true);
          }
        }}
      >
        <Icon color={ratingData.user_like ? 'primary' : ''}>
          <FontAwesomeIcon icon={faThumbsUp} />
        </Icon>
        {ratingData.likes != null && <p>{ratingData.likes}</p>}
      </Button>
      <Button
        onClick={() => {
          if (ratingData.user_like == false) {
            handleLikeRemove();
          } else {
            handleLike(false);
          }
        }}
      >
        <Icon
          color={
            ratingData.user_like == null
              ? ''
              : ratingData.user_like
              ? ''
              : 'primary'
          }
        >
          <FontAwesomeIcon icon={faThumbsDown} />
        </Icon>
        {ratingData.dislikes != null && <p>{ratingData.dislikes}</p>}
      </Button>
    </>
  );
};

export default Rating;
