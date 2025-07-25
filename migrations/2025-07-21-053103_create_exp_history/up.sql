-- Your SQL goes here

create table exp_history(
  id uuid primary key default gen_random_uuid(),
  user_id uuid references users(id),
  source varchar(20),
  amount integer,
  created_at timestamp not null default now()
);
