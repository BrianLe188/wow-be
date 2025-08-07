-- Your SQL goes here

create table user_place_access (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references users(id),
  place_id uuid not null references places(id),
  type varchar not null,
  created_at timestamp not null default now()
);
