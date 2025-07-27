-- This file should undo anything in `up.sql`

alter table reviews
drop column medias;
