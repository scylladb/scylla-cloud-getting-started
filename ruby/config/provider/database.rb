# frozen_string_literal: true

Application.register_provider(:database) do
  prepare do
    require 'cassandra'
  end

  start do
    cluster = Cassandra.cluster(
      username: ENV.fetch('DB_USER', nil),
      password: ENV.fetch('DB_PASSWORD', nil),
      hosts: ENV.fetch('DB_HOSTS', nil).split(',')
    )

    connection = cluster.connect

    Cli::Migrate.create_keyspace(session: connection) if Cli::Migrate.keyspace_exist?(
      session: connection
    )

    Cli::Migrate.create_table(session: connection, query: PLAYLIST_TABLE_QUERY) if Cli::Migrate.table_exist?(session: connection, table_name: PLAYLIST_TABLE_NAME)
    Cli::Migrate.create_table(session: connection, query: SONG_COUNTER_QUERY) if Cli::Migrate.table_exist?(session: connection, table_name: SONG_COUNTER_TABLE_NAME)

    connection = cluster.connect(KEYSPACE_NAME)

    register('database.connection', connection)
  end
end
