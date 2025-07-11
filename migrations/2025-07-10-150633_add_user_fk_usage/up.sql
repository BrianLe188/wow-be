-- Your SQL goes here

alter table feature_usages
add column user_id uuid not null references users(id);
