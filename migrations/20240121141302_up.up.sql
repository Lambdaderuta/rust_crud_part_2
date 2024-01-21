-- Add up migration script here
CREATE TABLE users (
    id serial primary key,
    email varchar,
    password_hash varchar
);