CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    author CHAR(36) NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT fk_author FOREIGN KEY(author) REFERENCES users(id)
);
CREATE TABLE tag_to_video (
    tag_id INT NOT NULL,
    video_id CHAR(37) NOT NULL,
    CONSTRAINT fk_tag FOREIGN KEY(tag_id) REFERENCES tags(id),
    CONSTRAINT fk_video FOREIGN KEY(video_id) REFERENCES videos(id),
    PRIMARY KEY(tag_id, video_id)
)