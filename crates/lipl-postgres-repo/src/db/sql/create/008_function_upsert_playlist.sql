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