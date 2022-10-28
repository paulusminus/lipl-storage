CREATE TABLE IF NOT EXISTS lyric (
    id UUID PRIMARY KEY,
    title VARCHAR UNIQUE NOT NULL,
    sub_title VARCHAR,
    parts VARCHAR
);

CREATE TABLE IF NOT EXISTS playlist (
    id UUID PRIMARY KEY,
    title VARCHAR UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS member (
    id SERIAL PRIMARY KEY,
    lyric_id UUID NOT NULL REFERENCES lyric ON DELETE CASCADE,
    playlist_id UUID NOT NULL REFERENCES playlist ON DELETE CASCADE,
    ordering INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS member_lyric_id ON member (lyric_id);

CREATE INDEX IF NOT EXISTS member_playlist_id ON member (playlist_id);

CREATE OR REPLACE view membership AS
    SELECT l.id AS lyric_id, l.title AS lyric_title, p.id AS playlist_id, p.title AS playlist_title, m.ordering as ordering FROM lyric l 
    INNER JOIN member m ON l.id = m.lyric_id 
    INNER JOIN playlist p ON p.id = m.playlist_id;

CREATE OR REPLACE FUNCTION set_members(p_id uuid, l_ids uuid[], out created_ids uuid[]) AS $$
DECLARE
    counter integer := 0;
    l_id uuid;
BEGIN
    RAISE NOTICE 'Start deleting members';
    DELETE FROM member WHERE playlist_id = p_id;
    RAISE NOTICE 'Finished deleting members';
    FOREACH l_id IN ARRAY l_ids
    LOOP
        counter := counter + 1;
        RAISE NOTICE 'Adding lyric with id %', l_id;
        RAISE NOTICE 'Counter = %', counter;
        BEGIN
            INSERT INTO MEMBER (playlist_id, lyric_id, ordering) VALUES (p_id, l_id, counter);
            created_ids := created_ids || l_id;
        EXCEPTION WHEN SQLSTATE '23503' THEN -- Do nothing and continue with next l_id
        END;
    END LOOP;

END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION fn_upsert_playlist(new_id uuid, new_title text, new_members uuid[], out created_ids uuid[]) AS $$
DECLARE
    l_id uuid;
    counter integer := 0;
BEGIN
    RAISE NOTICE 'Update id and title on playlist';
    counter := 0;
    INSERT INTO playlist (id, title)
    VALUES(new_id, new_title)
    ON CONFLICT (id)
    DO
    UPDATE SET title = new_title;

    RAISE NOTICE 'Start deleting members';
    DELETE FROM member WHERE playlist_id = new_id;
    RAISE NOTICE 'Finished deleting members';
    FOREACH l_id IN ARRAY new_members
    LOOP
        counter := counter + 1;
        RAISE NOTICE 'Adding lyric with id %', l_id;
        RAISE NOTICE 'Counter = %', counter;
        BEGIN
            INSERT INTO MEMBER (playlist_id, lyric_id, ordering) VALUES (new_id, l_id, counter);
            created_ids := created_ids || l_id;
        EXCEPTION WHEN SQLSTATE '23503' THEN -- Do nothing and continue with next l_id
        END;
    END LOOP;

END;
$$ LANGUAGE plpgsql;
