CREATE TABLE videos (
    id CHAR(37) UNIQUE PRIMARY KEY,
    user_id INT,
    title TEXT NOT NULL,
    description TEXT,
    creation_date DATE NOT NULL DEFAULT CURRENT_DATE,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
)