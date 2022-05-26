CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
    id uuid DEFAULT uuid_generate_v4 (),
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    PRIMARY KEY (id)
) 