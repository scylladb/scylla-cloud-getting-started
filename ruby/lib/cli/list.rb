# frozen_string_literal: true

module CLI
  class ListSongsCommand
    def initialize
      @repo = Application['database.connection']
    end

    def call
      query = <<~SQL
        SELECT * FROM #{KEYSPACE_NAME}.#{TABLE_NAME};
      SQL

      @repo.execute_async(query).join.rows
    end
  end
end
