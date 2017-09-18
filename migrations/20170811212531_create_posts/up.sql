-- Your SQL goes here
CREATE TABLE users (
       id SERIAL PRIMARY KEY,
       username VARCHAR NOT NULL
);

CREATE TABLE words (
       id SERIAL PRIMARY KEY,
       text_representation VARCHAR NOT NULL,
       language_specific_word_data VARCHAR
);

CREATE TABLE languages (
       id VARCHAR PRIMARY KEY,
       word_properties_json VARCHAR
);

CREATE TABLE user_encountered_words (
       word_id int4 NOT NULL REFERENCES words(id) PRIMARY KEY,
       definition VARCHAR NOT NULL,
       user_knowledge_level int4 NOT NULL
);

CREATE TABLE articles (
       id SERIAL NOT NULL PRIMARY KEY,
       title VARCHAR NOT NULL,
       text VARCHAR NOT NULL,
       language_name VARCHAR NOT NULL REFERENCES languages(id),
       unique_word_ids int4[]
);

CREATE TABLE user_data (
       user_id int4 NOT NULL REFERENCES users(id) PRIMARY KEY,
       article_ids int4[] NOT NULL DEFAULT '{}',
       users_word_ids int4[] NOT NULL DEFAULT '{}' 
);
