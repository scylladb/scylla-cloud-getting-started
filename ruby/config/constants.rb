# frozen_string_literal: true

KEYSPACE_NAME = 'media_player'
TABLE_NAME = 'playlist'

HELP_MESSAGE = <<~MSG
  Available commands:
    !add - add a new song
    !list - list all registered songs
    !delete - delete a specific song
    !stress - stress testing with mocked data
    !q - Quit the console
MSG
