# frozen_string_literal: true

module Cli
  class StressTestingCommand
    TOTAL_RECORDS = 100_000
    BATCH_SIZE = 1_000

    def initialize
      @repo = Application['database.connection']
    end

    def call
      puts <<~DESC
        ------------------------------------
        Inserting #{TOTAL_RECORDS} records into the database...
        >    Starting...
      DESC

      start = Time.now
      query = <<~SQL
        INSERT INTO #{KEYSPACE_NAME}.#{PLAYLIST_TABLE_NAME} (id,title,artist,album,created_at) VALUES (now(),?,?,?,?);
      SQL

      (TOTAL_RECORDS / BATCH_SIZE).times do
        futures = BATCH_SIZE.times.map do
          @repo.execute_async(query, arguments: ['Test Song', 'Test Artist', 'Test Album', Time.now])
        end
        futures.each(&:join)
      end

      puts "Time taken: #{Time.now - start}"
    end
  end
end
