# frozen_string_literal: true

module CLI
  class AddSongCommand
    def initialize
      @repo = Application['database.connection']
    end

    def call
      p @repo
    end
  end
end
