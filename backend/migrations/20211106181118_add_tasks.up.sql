CREATE TABLE task(
  id TEXT NOT NULL,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  state TEXT NOT NULL,
  pin INTEGER NOT NULL,

  PRIMARY KEY(id),
  CHECK(state IN ('pending', 'doing', 'done'))
);
