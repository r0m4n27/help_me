CREATE TABLE temp AS
SELECT id, title, body, state, pin
FROM task;

DROP TABLE task;

CREATE TABLE task(
  id TEXT NOT NULL,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  state TEXT NOT NULL,
  pin INTEGER NOT NULL,

  PRIMARY KEY(id),
  CHECK(state IN ('pending', 'doing', 'done'))
);

-- Set default value for created at
INSERT INTO task(id, title, body, state, pin)
SELECT id, title, body, state, pin
FROM temp;

DROP TABLE temp;
