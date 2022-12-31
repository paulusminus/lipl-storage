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
