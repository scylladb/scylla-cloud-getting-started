# frozen_string_literal: true

require_relative 'config/constants'
require_relative 'config/application'
require_relative 'config/provider/database'

Application.finalize!

# TODO: accept username, password and the 3 nodes as argument
# TODO: instantiate a scylla connection with it (using dry-system)
# TODO: migrate with a initial keyspace + table for the songs
settings = Cli::ArgsParser.call(ARGV)

ENV['DB_USER'] = settings[:username]
ENV['DB_PASSWORD'] = settings[:password]
ENV['DB_HOSTS'] = settings[:nodes]

puts HELP_MESSAGE

loop do
  command = Cli::GetCommand.call(HELP_MESSAGE)

  case command
  in '!add'
    print 'Insert song title: '
    song_name = $stdin.gets.chomp

    print 'Insert album name: '
    album = $stdin.gets.chomp

    print 'Insert artist name: '
    artist = $stdin.gets.chomp

    Cli::AddSongCommand.new.call(title: song_name, album:, artist:)
  in '!list'
    Cli::ListSongsCommand.new.call
  in '!delete'
    Cli::DeleteSongCommand.new.call
  in '!stress'
    Cli::StressTestingCommand.new.call
  in '!q'
    puts 'May the force be with you!'
    break
  else
    puts HELP_MESSAGE
  end
end
