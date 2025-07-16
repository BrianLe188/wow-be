-- Your SQL goes here

create table reviews (
  id uuid primary key default gen_random_uuid(),
  user_id uuid references users(id),
  place_id uuid not null references places(id),
  author_name text,
  author_url text,
  language varchar(2),
  profile_photo_url text,
  rating float not null,
  relative_time_description text,
  text text not null,
  time integer,
  created_at timestamp not null default now()
);
