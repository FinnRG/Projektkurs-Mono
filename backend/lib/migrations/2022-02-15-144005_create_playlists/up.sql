CREATE TABLE playlists (
    id CHAR(36) PRIMARY KEY,
    title VARCHAR NOT NULL,
    author CHAR(36) NOT NULL,
    CONSTRAINT fk_author FOREIGN KEY(author) REFERENCES users(id)
);

CREATE TABLE playlist_to_video (
    id SERIAL PRIMARY KEY,
    playlist_id CHAR(36) NOT NULL,
    video_id CHAR(37) NOT NULL,
    CONSTRAINT fk_playlist FOREIGN KEY(playlist_id) REFERENCES playlists(id),
    CONSTRAINT fk_video FOREIGN KEY(video_id) REFERENCES videos(id)
)