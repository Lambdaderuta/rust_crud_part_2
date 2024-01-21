-- Add up migration script here
CREATE TABLE products (
    id serial primary key,
    name varchar,
    color varchar,
    category varchar,
    price int
);