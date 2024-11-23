DELETE FROM auth.pending_users;
DELETE FROM auth.users;
ALTER TABLE auth.pending_users ALTER COLUMN cid TYPE text;
ALTER TABLE auth.users ALTER COLUMN cid TYPE text;
