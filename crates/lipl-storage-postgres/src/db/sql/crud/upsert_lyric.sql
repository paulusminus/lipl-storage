INSERT INTO lyric (id, title, parts)
VALUES($1, $2, $3)
ON CONFLICT (id)
DO
  UPDATE SET title = $2, parts = $3;
