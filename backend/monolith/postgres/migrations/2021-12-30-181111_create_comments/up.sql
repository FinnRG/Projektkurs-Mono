CREATE TABLE comments(
    id SERIAL PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    video_id CHAR(37) NOT NULL,
    content TEXT NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id),
    CONSTRAINT fk_video FOREIGN KEY(video_id) REFERENCES videos(id)
)