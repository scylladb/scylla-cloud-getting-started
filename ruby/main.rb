# frozen_string_literal: true

require_relative 'lib/cli/get_command'
require_relative 'lib/cli/add'
require_relative 'lib/cli/delete'
require_relative 'lib/cli/list'
require_relative 'lib/cli/stress_test'

HELP_MESSAGE = <<~MSG
  Available commands:
    !add - add a new song
    !list - list all registered songs
    !delete - delete a specific song
    !stress - stress testing with mocked data
MSG

puts HELP_MESSAGE

# TODO: accept username, password and the 3 nodes as argument
# TODO: instantiate a scylla connection with it (using dry-system)
# TODO: migrate with a initial keyspace + table for the songs

loop do
  command = CLI.get_command(HELP_MESSAGE)

  case command
  in '!add'
    CLI::AddSongCommand.new.call
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
