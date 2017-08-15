-- Your SQL goes here
CREATE TABLE users (
       id SERIAL PRIMARY KEY,
       username VARCHAR NOT NULL
);

CREATE TABLE articles (
       id SERIAL PRIMARY KEY,
       user_id int4 NOT NULL REFERENCES users(id),
       title VARCHAR NOT NULL,
       text TEXT NOT NULL
);

