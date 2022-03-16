DROP TABLE IF EXISTS videos;
CREATE TABLE videos (
    id CHAR(37) PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
)