CREATE TABLE videos (
    id CHAR(36) UNIQUE PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    author CHAR(36) NOT NULL,
    date TEXT NOT NULL,
    visibility INT NOT NULL,
    status INT NOT NULL
)