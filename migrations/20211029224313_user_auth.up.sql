CREATE TABLE user(
  user_name TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  user_type TEXT NOT NULL,

PRIMARY KEY(user_name),
  CHECK (user_type in ('admin', 'tutor'))
);

CREATE TRIGGER check_only_one_admin
  BEFORE INSERT
  ON user
BEGIN
  SELECT
  CASE
    WHEN (SELECT count(*) FROM user WHERE user_type = 'admin') >= 1 THEN
      RAISE (ABORT, 'Admin already exists')
  END;
END;


CREATE TABLE user_token(
  user_name TEXT NOT NULL,
  token TEXT NOT NULL,
  expiry TEXT NOT NULL,

  PRIMARY KEY(token),
  FOREIGN KEY (user_name) REFERENCES user(user_name)
);
