# frozen_string_literal: true

require 'uuid'

module Cli
  class AddSongCommand
    def initialize
      @repo = Application['database.connection']
    end

    # @param title [String]
    # @param album [String]
    # @param artist [String]
    def call(title:, album:, artist:)
      playlist_insert_query = <<~SQL
        INSERT INTO #{KEYSPACE_NAME}.#{PLAYLIST_TABLE_NAME} (id,title,artist,album,created_at) VALUES (now(),?,?,?,?);
      SQL

      @repo.execute_async(playlist_insert_query, arguments: [title, artist, album, Time.now]).join

      inserted_song = last_song

      update_counter_query = <<~SQL
        UPDATE #{KEYSPACE_NAME}.#{SONG_COUNTER_TABLE_NAME} SET times_played = times_played + 1 WHERE song_id = ?
      SQL

      @repo.execute_async(update_counter_query, arguments: [inserted_song['id']]).join

      puts "Song '#{title}' from artist '#{artist}' Added!"
    end

    # TODO: investigate a way to get the inserted ID from the insert into query and remove this select.
    def last_song
      query = <<~SQL
        SELECT * FROM #{KEYSPACE_NAME}.#{PLAYLIST_TABLE_NAME} LIMIT 1;
      SQL

      @repo.execute_async(query).join.rows.first
    end
  end
end
