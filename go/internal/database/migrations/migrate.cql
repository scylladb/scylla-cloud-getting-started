CREATE KEYSPACE IF NOT EXISTS media_player;

CREATE TABLE IF NOT EXISTS media_player.playlist
(
    id uuid,
    title text,
    album text,
    artist text,
    created_at timestamp,
    PRIMARY KEY (id, created_at)
) WITH CLUSTERING ORDER BY (created_at DESC);

CREATE TABLE IF NOT EXISTS media_player.song_counter
(
    song_id uuid,
    times_played counter,
    PRIMARY KEY (song_id)
);
