-- Your SQL goes here

create table missions(
  id uuid primary key default gen_random_uuid(),
  code varchar(20) unique not null,
  name varchar(50) not null,
  description text,
  exp_reward integer not null,
  gift_reward_count integer,
  gift_reward_type varchar(20),
  max_per_day integer default 1,
  created_at timestamp not null default now()
);
