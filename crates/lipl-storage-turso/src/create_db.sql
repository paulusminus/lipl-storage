CREATE TABLE IF NOT EXISTS lyric (
    id VARCHAR PRIMARY KEY,
    title VARCHAR UNIQUE NOT NULL,
    sub_title VARCHAR,
    parts VARCHAR
);

CREATE TABLE IF NOT EXISTS playlist (
    id VARCHAR PRIMARY KEY,
    title VARCHAR UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS member (
    playlist_id VARCHAR NOT NULL REFERENCES playlist ON DELETE CASCADE,
    lyric_id VARCHAR NOT NULL REFERENCES lyric ON DELETE CASCADE,
    ordering INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS member_lyric_id ON member (lyric_id);

CREATE INDEX IF NOT EXISTS member_playlist_id ON member (playlist_id, ordering);
