-- This file should undo anything in `up.sql`

alter table users 
drop column level,
drop column exp;
