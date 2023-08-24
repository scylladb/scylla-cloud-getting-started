# frozen_string_literal: true

module Cli
  class DeleteSongCommand
    def initialize
      @repo = Application['database.connection']
    end

    def call
      songs = Cli::ListSongsCommand.new.call

      song_to_delete_index = select_song_to_delete(songs:)

      query = <<~SQL
        DELETE FROM #{KEYSPACE_NAME}.#{PLAYLIST_TABLE_NAME} WHERE id = ?
      SQL

      song_to_delete = songs.to_a[song_to_delete_index]

      @repo.execute_async(query, arguments: [song_to_delete['id']]).join
    end

    private

    # TODO: Maybe this should be on a different class, because it deals with the outside world (IO)
    # @param songs [Array<Hash>]
    # @return [Integer]
    def select_song_to_delete(songs:)
      songs.each_with_index do |song, index|
        puts <<~DESC
          Index: #{index + 1}  | Song: #{song['title']} | Album: #{song['album']} | Artist: #{song['artist']} | Created At: #{song['created_at']}
        DESC
      end

      # TODO: maybe this could be on another class?
      print 'Select a index to be deleted: '
      $stdin.gets.chomp.to_i - 1
    end
  end
end
