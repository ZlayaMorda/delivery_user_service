--USERS

ALTER TABLE users ALTER COLUMN created_at SET DEFAULT current_timestamp;

CREATE FUNCTION set_users_timestamp()
RETURNS trigger
AS
  $set_users_timestamp$
BEGIN
  IF (tg_op = 'UPDATE') THEN
    UPDATE users
    SET    updated_at = current_timestamp
    WHERE  user_uuid = NEW.user_uuid;

  END IF;
  RETURN NULL;
END;
$set_users_timestamp$ LANGUAGE plpgsql;

CREATE TRIGGER set_users_timestamp AFTER UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION set_users_timestamp();

-- COURIERS

ALTER TABLE couriers ALTER COLUMN created_at SET DEFAULT current_timestamp;

CREATE FUNCTION set_couriers_timestamp()
RETURNS trigger
AS
  $set_couriers_timestamp$
BEGIN
  IF (tg_op = 'UPDATE') THEN
    UPDATE couriers
    SET    updated_at = current_timestamp
    WHERE  user_uuid = NEW.user_uuid;

  END IF;
  RETURN NULL;
END;
$set_couriers_timestamp$ LANGUAGE plpgsql;

CREATE TRIGGER set_couriers_timestamp AFTER UPDATE ON couriers
    FOR EACH ROW EXECUTE FUNCTION set_couriers_timestamp();