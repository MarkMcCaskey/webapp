-- Your SQL goes here
CREATE TABLE users (
       id SERIAL PRIMARY KEY,
       username VARCHAR NOT NULL,
       password_hash VARCHAR NOT NULL,
       salt VARCHAR NOT NULL
);

CREATE TABLE languages (
       id TEXT PRIMARY KEY,
       word_properties_json VARCHAR
);

CREATE TABLE words (
       id SERIAL PRIMARY KEY,
       text_representation VARCHAR NOT NULL,
       language_id TEXT NOT NULL REFERENCES languages(id),
       language_specific_word_data VARCHAR
);

CREATE TABLE articles (
       id SERIAL NOT NULL PRIMARY KEY,
       title VARCHAR NOT NULL,
       text VARCHAR NOT NULL,
       language_id TEXT NOT NULL REFERENCES languages(id)
);

CREATE TABLE article_words (
       article_id int4 NOT NULL REFERENCES articles(id),
       word_id int4 NOT NULL REFERENCES words(id),
       PRIMARY KEY(article_id, word_id)
);

CREATE TABLE user_articles (
       user_id int4 NOT NULL REFERENCES users(id),
       article_id int4 NOT NULL REFERENCES articles(id),
       PRIMARY KEY(user_id, article_id)
);

CREATE TABLE user_words (
       user_id int4 NOT NULL REFERENCES users(id),
       word_id int4 NOT NULL REFERENCES words(id),
       knowledge_level int4 NOT NULL,
       word_definition VARCHAR NOT NULL,
       PRIMARY KEY(user_id, word_id)
);
