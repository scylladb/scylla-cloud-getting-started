# frozen_string_literal: true

module Cli
  class DeleteSongCommand
    def initialize
      @repo = Application['database.connection']
    end

    def call
      songs = Cli::ListSongsCommand.new.call

      song_to_delete_index = select_song_to_delete(songs:)
    end

    private

    # @param songs [Array<Hash>]
    def select_song_to_delete(songs:)
      songs.each_with_index do |index, song|
        puts <<~DESC
          Index: #{index}  | Song: #{song[:title]} | Album: #{song[:album]} | Artist: #{song[:artist]} | Created At: #{song[:created_at]}
        DESC
      end

      # TODO: maybe this could be on another class?
      print 'Select a index to be deleted: '
    end
  end
end
