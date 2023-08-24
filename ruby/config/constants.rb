# frozen_string_literal: true

KEYSPACE_NAME = 'media_player'
PLAYLIST_TABLE_NAME = 'playlist'
SONG_COUNTER_TABLE_NAME = 'song_counter'

PLAYLIST_TABLE_QUERY = <<~SQL
  CREATE TABLE #{KEYSPACE_NAME}.#{PLAYLIST_TABLE_NAME} (
    id uuid,
    title text,
    album text,
    artist text,
    created_at timestamp,
    PRIMARY KEY (id, created_at)
  ) WITH CLUSTERING ORDER BY (created_at DESC);
SQL

SONG_COUNTER_QUERY = <<~SQL
  CREATE TABLE #{KEYSPACE_NAME}.#{SONG_COUNTER_TABLE_NAME} (
    song_id uuid,
    times_played counter,
    PRIMARY KEY (song_id)
  )
SQL

HELP_MESSAGE = <<~MSG
  Available commands:
    !add - add a new song
    !list - list all registered songs
    !delete - delete a specific song
    !stress - stress testing with mocked data
    !q - Quit the console
MSG
