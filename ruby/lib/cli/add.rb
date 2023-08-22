# frozen_string_literal: true

require 'securerandom'

module CLI
  class AddSongCommand
    def initialize
      @repo = Application['database.connection']
    end

    # @param title [String]
    # @param album [String]
    # @param artist [String]
    def call(title:, album:, artist:)
      query = <<~SQL
        INSERT INTO #{KEYSPACE_NAME}.#{TABLE_NAME} (id,title,artist,album,created_at) VALUES (?,?,?,?,?);
      SQL

      @repo.execute_async(query, arguments: [SecureRandom.uuid, title, artist, album, Time.now]).join

      puts "Song '#{title}' from artist '#{artist}' Added!"
    end
  end
end
