--USERS

ALTER TABLE users ALTER COLUMN created_at DROP DEFAULT;

DROP TRIGGER set_users_timestamp ON users;

DROP FUNCTION set_users_timestamp;
-- COURIERS

ALTER TABLE couriers ALTER COLUMN created_at DROP DEFAULT;

DROP TRIGGER set_couriers_timestamp ON couriers;

DROP FUNCTION set_couriers_timestamp;