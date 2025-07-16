-- This file should undo anything in `up.sql`

alter table places 
drop column range_time_view_count;
