INSERT INTO playlist (id, title)
VALUES($1, $2)
ON CONFLICT (id)
DO
  UPDATE SET title = $2;
