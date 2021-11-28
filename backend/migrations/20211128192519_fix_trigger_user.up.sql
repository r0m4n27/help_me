DROP TRIGGER check_only_one_admin;

CREATE TRIGGER check_only_one_admin
  BEFORE INSERT
  ON user
BEGIN
  SELECT
  CASE
    WHEN
      (SELECT count(*) FROM user WHERE user_type = 'admin') >= 1
      AND new.user_type = 'admin' THEN
      RAISE (ABORT, 'Admin already exists')
  END;
END;
