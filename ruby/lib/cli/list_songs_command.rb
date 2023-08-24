# frozen_string_literal: true

module Cli
  class ListSongsCommand
    def initialize
      @repo = Application['database.connection']
    end

    def call
      query = <<~SQL
        SELECT * FROM #{KEYSPACE_NAME}.#{PLAYLIST_TABLE_NAME};
      SQL

      @repo.execute_async(query).join.rows
    end
  end
end
