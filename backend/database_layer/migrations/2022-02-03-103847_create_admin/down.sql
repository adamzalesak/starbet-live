-- This file should undo anything in `up.sql`
DELETE FROM "user_address" WHERE user_id = 0;
DELETE FROM "user" WHERE id = 0;