CREATE OR REPLACE view membership AS
    SELECT l.id AS lyric_id, l.title AS lyric_title, p.id AS playlist_id, p.title AS playlist_title, m.ordering as ordering FROM lyric l 
    INNER JOIN member m ON l.id = m.lyric_id 
    INNER JOIN playlist p ON p.id = m.playlist_id;
