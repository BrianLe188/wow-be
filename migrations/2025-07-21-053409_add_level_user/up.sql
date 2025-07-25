-- Your SQL goes here

alter table users
add column level integer default 1,
add column exp integer default 0;
