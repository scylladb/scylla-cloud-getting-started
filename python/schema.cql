CREATE KEYSPACE IF NOT EXISTS prod_media_player
    WITH REPLICATION = {'class': 'NetworkTopologyStrategy', 'replication_factor': 3};

CREATE TABLE IF NOT EXISTS prod_media_player.songs (
    id uuid,
    title text,
    album text,
    artist text,
    release_year int,
    created_at timestamp,
    PRIMARY KEY (id, created_at)
);

CREATE TABLE IF NOT EXISTS prod_media_player.recently_played_songs (
    song_id uuid,
    listened_at timestamp,
    PRIMARY KEY (song_id, listened_at)
);

CREATE TABLE IF NOT EXISTS prod_media_player.played_songs_counter (
    song_id uuid,
    times_played counter,
    PRIMARY KEY (song_id)
);
