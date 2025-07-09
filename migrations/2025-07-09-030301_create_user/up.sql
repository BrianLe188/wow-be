-- Your SQL goes here

create table users (
  id uuid primary key default gen_random_uuid(),
  email varchar not null,
  password text not null,
  created_at timestamp not null default now()
);
