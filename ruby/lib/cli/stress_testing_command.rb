# frozen_string_literal: true

require 'async'

module Cli
  class StressTestingCommand
    def initialize
      @repo = Application['database.connection']
    end

    def call
      puts <<~DESC
        ------------------------------------
        Inserting 100.000 records into the database...
        >    Starting...
      DESC

      start = Time.now

      Async do |task|
        100_001.times do
          task.async do
            query = <<~SQL
              INSERT INTO #{KEYSPACE_NAME}.#{PLAYLIST_TABLE_NAME} (id,title,artist,album,created_at) VALUES (now(),?,?,?,?);
            SQL
            @repo.execute_async(query, arguments: ['Test Song', 'Test Artist', 'Test Album', Time.now]).join
          end
        end
      end

      puts "Time taken: #{Time.now - start}"
    end
  end
end
