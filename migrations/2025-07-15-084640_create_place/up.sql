-- Your SQL goes here

create table places (
  id uuid primary key default gen_random_uuid(),
  place_id text not null unique,
  name text not null,
  formatted_address text,
  formatted_phone_number text,
  business_status text,
  adr_address text,
  icon text,
  icon_background_color varchar(10),
  icon_mask_base_uri text,
  rating float,
  user_ratings_total int,
  url text,
  website text,
  vicinity text,
  utc_offset text,
  reference text,
  geometry jsonb,
  types text[],
  address_components jsonb,
  plus_code jsonb,
  created_at timestamp not null default now()
);

-- CREATE TABLE places (
--     place_id TEXT PRIMARY KEY,
--     name TEXT,
--     formatted_address TEXT,
--     formatted_phone_number TEXT,
--     international_phone_number TEXT,
--     business_status TEXT,
--     adr_address TEXT,
--     icon TEXT,
--     icon_background_color TEXT,
--     icon_mask_base_uri TEXT,
--     rating FLOAT,
--     user_ratings_total INT,
--     url TEXT,
--     website TEXT,
--     vicinity TEXT,
--     utc_offset INT,
--     reference TEXT,
--     geometry JSONB,
--     opening_hours JSONB,
--     photos JSONB,
--     reviews JSONB,
--     types TEXT[],                -- mảng string
--     address_components JSONB,
--     plus_code JSONB,
--     waypoint JSONB               -- nếu có custom waypoint
-- );
