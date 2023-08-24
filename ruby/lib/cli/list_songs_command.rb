# frozen_string_literal: true

module Cli
  class ListSongsCommand
    def initialize
      @repo = Application['database.connection']
    end

    def call
      playlist_insert_queryist_insert_query = <<~SQL
        SELECT * FROM #{KEYSPACE_NAME}.#{PLAYLIST_TABLE_NAME};
      SQL

      @repo.execute_async(playlist_insert_query).join.rows
    end
  end
end
