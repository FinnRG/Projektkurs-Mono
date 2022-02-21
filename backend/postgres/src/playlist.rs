use crate::*;
use models::{NewPlaylist, Playlist, PlaylistUpdate, VideoSuggestion};

pub fn create_playlist<'a>(
    conn: &PgConnection,
    id: &'a str,
    title: &'a str,
    author: &'a str,
) -> Playlist {
    use schema::playlists;

    let new_playlist = NewPlaylist { id, title, author };

    diesel::insert_into(playlists::table)
        .values(&new_playlist)
        .get_result(conn)
        .expect("Error creating new playlist")
}

fn get_author(conn: &PgConnection, playlist_id: &str) -> Result<String, diesel::result::Error> {
    use schema::playlists;

    playlists::table
        .find(playlist_id)
        .select(playlists::author)
        .first(conn)
}

macro_rules! return_on_unauthorized {
    ($conn: ident, $playlist_id: ident, $user_id: ident, $val: expr) => {
        let author = get_author($conn, $playlist_id);

        if (author.is_err() || author.unwrap() != $user_id) {
            return $val;
        }
    };
    ($conn: ident, $playlist_id: ident, $user_id: ident) => {
        return_on_unauthorized!($conn, $playlist_id, $user_id, ());
    };
}

pub fn add_video_to_playlist(conn: &PgConnection, playlist_id: &str, video_id: &str, author: &str) {
    use schema::playlist_to_video;

    return_on_unauthorized!(conn, playlist_id, author);

    diesel::insert_into(playlist_to_video::table)
        .values((
            playlist_to_video::playlist_id.eq(playlist_id),
            playlist_to_video::video_id.eq(video_id),
        ))
        .execute(conn)
        .expect("Unable to add video to playlist");

}

pub fn remove_video_from_playlist(
    conn: &PgConnection,
    playlist_id: &str,
    entry_id: i32,
    author: &str,
) {
    use schema::playlist_to_video;

    return_on_unauthorized!(conn, playlist_id, author);

    diesel::delete(playlist_to_video::table.find(entry_id))
        .execute(conn)
        .expect("Unable to remove relationship for playlist and video");
}

pub fn delete_playlist(conn: &PgConnection, playlist_id: &str, author: &str) {
    use schema::playlists;

    return_on_unauthorized!(conn, playlist_id, author);

    diesel::delete(playlists::table.find(playlist_id))
        .execute(conn)
        .expect("Unable to delete playlist");
}

pub fn get_videos_for_playlist(
    conn: &PgConnection,
    playlist_id: &str,
    author: &str,
) -> Vec<VideoSuggestion> {
    use schema::playlist_to_video;
    use schema::videos;

    return_on_unauthorized!(conn, playlist_id, author, vec![]);

    playlist_to_video::table
        .filter(playlist_to_video::playlist_id.eq(playlist_id))
        .inner_join(videos::table)
        .select((
            playlist_to_video::video_id,
            videos::title,
            videos::description,
        ))
        .load::<VideoSuggestion>(conn)
        .expect("Unable to fetch videos for playlist")
}

pub fn update_playlist<'a>(conn: &PgConnection, playlist_id: &str, title: Option<&'a str>, author: &str) {
    use schema::playlists;

    return_on_unauthorized!(conn, playlist_id, author);

    diesel::update(playlists::table.find(playlist_id))
        .set(&PlaylistUpdate { title })
        .execute(conn)
        .expect("Unable to update playlist");
}
