-- Your SQL goes here

create table action_count(
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references users(id),
  review_place integer default 0,
  created_at timestamp not null default now()
);
