SELECT p.id, p.title, ARRAY(SELECT lyric_id FROM member WHERE playlist_id = p.id ORDER By ordering) AS members from Playlist p WHERE p.id = $1;