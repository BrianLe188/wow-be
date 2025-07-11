-- This file should undo anything in `up.sql`

alter table feature_usages
drop column user_id;
