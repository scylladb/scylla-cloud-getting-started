# frozen_string_literal: true

module Cli
  class Migrate
    # @param session [Cassandra#Cluster]
    # @return [Boolean]
    def self.keyspace_exist?(session:)
      has_keyspace = session.execute_async('select keyspace_name from system_schema.keyspaces WHERE keyspace_name=?',
                                           arguments: [KEYSPACE_NAME]).join.rows.size

      has_keyspace.zero?
    end

    # @param session [Cassandra#Cluster]
    # @param table_name [String]
    # @return [Boolean]
    def self.table_exist?(session:, table_name:)
      has_table = session.execute_async(
        'select keyspace_name, table_name from system_schema.tables where keyspace_name = ? AND table_name = ?', arguments: [KEYSPACE_NAME, table_name]
      ).join.rows.size

      has_table.zero?
    end

    # @param session [Cassandra#Cluster]
    # @param keyspace_name [String]
    # @return [void]
    def self.create_keyspace(session:)
      new_keyspace_query = <<~SQL
        CREATE KEYSPACE #{KEYSPACE_NAME}
        WITH replication = {
          'class': 'NetworkTopologyStrategy',
          'replication_factor': '3'
        }
        AND durable_writes = true
      SQL

      session.execute_async(new_keyspace_query).join
    end

    # @param session [Cassandra#Cluster]
    # @param query [String]
    # @return [void]
    def self.create_table(session:, query:)
      session.execute_async(query).join
    end
  end
end
