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

CREATE OR REPLACE FUNCTION fn_upsert_lyric(new_id uuid, new_title text, new_parts text)
RETURNS TABLE (
    id uuid,
    title text,
    parts text
) AS $$
BEGIN
    INSERT INTO lyric (id, title, parts)
    VALUES (new_id, new_title, new_parts)
    ON CONFLICT ON CONSTRAINT lyric_pkey
    DO
    UPDATE SET title = new_title, parts = new_parts;
    RETURN QUERY SELECT new_id AS id, new_title AS title, new_parts AS parts;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION fn_upsert_playlist(new_id uuid, new_title text, new_members uuid[]) 
RETURNS TABLE (
    id uuid,
    title text,
    members uuid[]
) AS $$
DECLARE
    l_id uuid;
    counter integer := 0;
    members uuid[];
BEGIN
    counter := 0;
    INSERT INTO playlist (id, title)
    VALUES(new_id, new_title)
    ON CONFLICT ON CONSTRAINT playlist_pkey
    DO
    UPDATE SET title = new_title;

    DELETE FROM member WHERE playlist_id = new_id;
    RAISE NOTICE 'Members deleted';
    FOREACH l_id IN ARRAY new_members
    LOOP
        counter := counter + 1;
        BEGIN
            INSERT INTO MEMBER (playlist_id, lyric_id, ordering) VALUES (new_id, l_id, counter);
            members := members || l_id;
        END;
        RAISE NOTICE 'Lyric with id % added', l_id;
    END LOOP;

    RETURN QUERY SELECT new_id AS id, new_title AS title, members AS members;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION fn_playlist_item(selected_id uuid)
RETURNS TABLE (
    id uuid,
    title text,
    members uuid[]
) AS $$
DECLARE
    l_id uuid;
BEGIN
    RAISE NOTICE 'Assisgn select_id to id';
    id := select_id;

    RAISE NOTICE 'Select title from playlist';
    SELECT title INTO title FROM playlist WHERE playlist.id = selected_id;
    SELECT lyric_id INTO members FROM member WHERE playlist_id = selected_id ORDER BY ordering;
END;
$$ LANGUAGE plpgsql;
