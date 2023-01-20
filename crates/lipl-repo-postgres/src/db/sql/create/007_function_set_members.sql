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