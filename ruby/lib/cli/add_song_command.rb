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
      query = <<~SQL
        INSERT INTO #{KEYSPACE_NAME}.#{TABLE_NAME} (id,title,artist,album,created_at) VALUES (now(),?,?,?,?);
      SQL

      body = [title, artist, album, Time.now]

      @repo.execute_async(query, arguments: body).join

      puts "Song '#{title}' from artist '#{artist}' Added!"
    end
  end
end
