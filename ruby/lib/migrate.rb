# frozen_string_literal: true

class MigrationUtils
  # @param session [Cassandra#Cluster]
  # @param keyspace_name [String]
  # @return [Boolean]
  def self.keyspace_exist?(session:, keyspace_name:)
    has_keyspace = session.execute_async('select keyspace_name from system_schema.keyspaces WHERE keyspace_name=?',
                                         arguments: [keyspace_name]).join.rows.size

    has_keyspace.zero?
  end

  # @param session [Cassandra#Cluster]
  # @param keyspace_name [String]
  # @param table_name [String]
  # @return [Boolean]
  def self.table_exist?(session:, keyspace_name:, table_name:)
    has_table = session.execute_async(
      'select keyspace_name, table_name from system_schema.tables where keyspace_name = ? AND table_name = ?', arguments: [keyspace_name, table_name]
    ).join.rows.size

    has_table.zero?
  end

  # @param session [Cassandra#Cluster]
  # @param keyspace_name [String]
  # @return [void]
  def self.create_keyspace(session:, keyspace_name:)
    new_keyspace_query = <<~SQL
      CREATE KEYSPACE #{keyspace_name}
      WITH replication = {
        'class': 'NetworkTopologyStrategy',
        'replication_factor': '3'
      }
      AND durable_writes = true
    SQL

    session.execute_async(new_keyspace_query).join
  end

  # @param session [Cassandra#Cluster]
  # @param keyspace_name [String]
  # @param table_name [String]
  # @return [void]
  def self.create_table(session:, keyspace_name:, table_name:)
    new_table_query = <<~SQL
      CREATE TABLE #{keyspace_name}.#{table_name} (
        id uuid,
        title text,
        album text,
        artist text,
        created_at timestamp,
        PRIMARY KEY (id, created_at)
      )
    SQL

    session.execute_async(new_table_query).join
  end
end
