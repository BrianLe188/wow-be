-- Your SQL goes here

create table subscriptions (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references users(id),
  environment varchar(255) not null default '',
  orig_tx_id varchar(255) not null default '',
  latest_receipt text not null,
  start_date timestamp not null default now(),
  end_date timestamp not null default now(),
  app varchar(255) not null default '',
  product_id varchar(255) not null default '',
  is_cancelled boolean not null,
  validation_response text not null,
  fake boolean not null default false, 
  created_at timestamp not null default now()
);
