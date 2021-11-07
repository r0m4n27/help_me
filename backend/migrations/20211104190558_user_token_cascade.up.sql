-- https://stackoverflow.com/questions/1884818/how-do-i-add-a-foreign-key-to-an-existing-sqlite-table
CREATE TABLE temp AS
SELECT user_name, token, expiry
FROM user_token;

DROP TABLE user_token;

CREATE TABLE user_token(
  user_name TEXT NOT NULL,
  token TEXT NOT NULL,
  expiry TEXT NOT NULL,

  PRIMARY KEY(token),
  FOREIGN KEY (user_name) REFERENCES user(user_name)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);

INSERT INTO user_token(user_name, token, expiry)
SELECT user_name, token, expiry
FROM temp;

DROP TABLE temp;
