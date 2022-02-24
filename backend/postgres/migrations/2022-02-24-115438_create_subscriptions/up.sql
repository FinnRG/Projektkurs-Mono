CREATE TABLE subscriptions (
    user_id CHAR(36),
    tag_id INT,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id),
    CONSTRAINT fk_tag FOREIGN KEY(tag_id) REFERENCES tags(id),
    PRIMARY KEY(user_id, tag_id)
);
CREATE TABLE notifications (
    user_id CHAR(36),
    tag_id INT,
    video_id CHAR(37) NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id),
    CONSTRAINT fk_tag FOREIGN KEY(tag_id) REFERENCES tags(id),
    CONSTRAINT fk_video FOREIGN KEY(video_id) REFERENCES videos(id),
    PRIMARY KEY(user_id, tag_id, video_id)
)