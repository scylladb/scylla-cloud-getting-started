# frozen_string_literal: true

module Cli
  class ArgsParser
    # @param args [Array<String>] This is usually just the ARGV variable.
    # @return [Hash<String, String>] After the parsing we'll return a hash with named params based on the order
    # @example
    # Cli::ArgsParser.call(ARGV) # => {username: 'username', password: 'password', nodes: 'node1,node2'}
    def self.call(args)
      username = args[0]
      password = args[1]
      nodes = args[2]

      { username:, password:, nodes: }
    end
  end
end
