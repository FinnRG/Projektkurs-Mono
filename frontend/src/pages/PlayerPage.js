import { Navigate, Outlet, useParams } from 'react-router';

const PlayerPage = () => {
  const params = useParams();

  let last_video_id = null;
  if (!params.video_id) {
    last_video_id = localStorage.getItem('last_video_id');
  }

  return (
    <>
      <Outlet />
      {last_video_id && <Navigate to={'/player/' + last_video_id} replace />}
    </>
  );
};

export default PlayerPage;
