# frozen_string_literal: true

Application.register_provider(:database) do
  prepare do
    require 'cassandra'
    require_relative '../../lib/migrate'
  end

  start do
    cluster = Cassandra.cluster(
      username: ENV.fetch('DB_USER', nil),
      password: ENV.fetch('DB_PASSWORD', nil),
      hosts: ENV.fetch('DB_HOSTS', nil).split(',')
    )

    connection = cluster.connect

    keyspace_name = KEYSPACE_NAME
    table_name = TABLE_NAME

    MigrationUtils.create_keyspace(session: connection, keyspace_name:) if MigrationUtils.keyspace_exist?(
      session: connection, keyspace_name:
    )
    MigrationUtils.create_table(session: connection, keyspace_name:, table_name:) if MigrationUtils.table_exist?(session: connection,
                                                                                                                 keyspace_name:, table_name:)

    connection = cluster.connect(keyspace_name)

    register('database.connection', connection)
  end
end
