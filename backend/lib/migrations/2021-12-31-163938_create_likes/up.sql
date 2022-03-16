CREATE TABLE likes (
    value BOOLEAN NOT NULL,
    user_id CHAR(36) NOT NULL,
    video_id CHAR(37) NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id),
    CONSTRAINT fk_video FOREIGN KEY(video_id) REFERENCES videos(id),
    PRIMARY KEY(user_id, video_id)
)