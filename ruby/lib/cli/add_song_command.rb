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
        INSERT INTO #{KEYSPACE_NAME}.#{TABLE_NAME} (id,title,artist,album,created_at) VALUES (?,?,?,?,?);
      SQL

      # TODO: Fix this uuid error
      # /Users/cherryramatis/.asdf/installs/ruby/3.2.2/lib/ruby/gems/3.2.0/gems/cassandra-driver-3.2.5/lib/cassandra/future.rb:637:in `get': Exception while binding column id: marshaling error: Validation failed for uuid - got 36 bytes (Cassandra::Errors::InvalidError)
      # from /Users/cherryramatis/.asdf/installs/ruby/3.2.2/lib/ruby/gems/3.2.0/gems/cassandra-driver-3.2.5/lib/cassandra/future.rb:402:in `get'
      # from /Users/cherryramatis/Repos/scylla-cloud-getting-started/ruby/lib/cli/add_song_command.rb:23:in `call'
      # from main.rb:34:in `block in <main>'
      # from main.rb:20:in `loop'
      # from main.rb:20:in `<main>'
      body = ['a4a70900-24e1-11df-8924-001ff3591711', title, artist, album, Time.now]

      p body

      @repo.execute_async(query, arguments: body).join

      puts "Song '#{title}' from artist '#{artist}' Added!"
    end
  end
end
