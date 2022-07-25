CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    login VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    full_name VARCHAR NOT NULL
)