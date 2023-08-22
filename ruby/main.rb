# frozen_string_literal: true

require_relative 'lib/cli/get_command'
require_relative 'lib/cli/add'
require_relative 'lib/cli/delete'
require_relative 'lib/cli/list'
require_relative 'lib/cli/stress_test'
require_relative 'lib/cli/parse_args'
require_relative 'config/application'
require_relative 'config/provider/database'

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

# TODO: accept username, password and the 3 nodes as argument
# TODO: instantiate a scylla connection with it (using dry-system)
# TODO: migrate with a initial keyspace + table for the songs
settings = CLI::ArgsParser.call(ARGV)

ENV['DB_USER'] = settings[:username]
ENV['DB_PASSWORD'] = settings[:password]
ENV['DB_HOSTS'] = settings[:nodes]

Application.finalize!

puts HELP_MESSAGE

loop do
  command = CLI.get_command(HELP_MESSAGE)

  case command
  in '!add'
    print 'Insert song title: '
    song_name = $stdin.gets.chomp

    print 'Insert album name: '
    album = $stdin.gets.chomp

    print 'Insert artist name: '
    artist = $stdin.gets.chomp

    CLI::AddSongCommand.new.call(title: song_name, album:, artist:)
  in '!list'
    CLI::ListSongsCommand.new.call
  in '!delete'
    CLI::DeleteSongCommand.new.call
  in '!stress'
    CLI::StressTestingCommand.new.call
  in '!q'
    puts 'May the force be with you!'
    break
  else
    puts HELP_MESSAGE
  end
end
