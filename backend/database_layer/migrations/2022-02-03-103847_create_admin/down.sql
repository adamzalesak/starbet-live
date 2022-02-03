-- This file should undo anything in `up.sql`
DELETE FROM "user_address" WHERE user_id = 1;
DELETE FROM "user" WHERE id = 1;