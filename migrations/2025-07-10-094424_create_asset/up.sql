-- Your SQL goes here

create table feature_usages (
  id uuid primary key default gen_random_uuid(),
  route_calculation_count integer not null default 10,
  created_at timestamp not null default now()
);
