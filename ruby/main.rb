# frozen_string_literal: true

require_relative 'config/constants'
require_relative 'config/application'
require_relative 'config/provider/database'

settings = Cli::ArgsParser.call(ARGV)

ENV['DB_USER'] = settings[:username]
ENV['DB_PASSWORD'] = settings[:password]
ENV['DB_HOSTS'] = settings[:nodes]

Application.finalize!

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
    songs = Cli::ListSongsCommand.new.call

    songs.each do |song|
      puts "ID: #{song['id']} | Song: #{song['title']} | Album: #{song['album']} | Created At: #{song['created_at']}"
    end
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
