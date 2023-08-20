# frozen_string_literal: true

Application.register_provider(:database) do
  prepare do
    require 'cassandra'
  end

  start do
    # TODO: don't know if this is the best architecture choice
    # Maybe the provider didn't need to understand the outside world with the ARGV.
    cluster = Cassandra.cluster(
      username: ENV.fetch('DB_USER', nil),
      password: ENV.fetch('DB_PASSWORD', nil),
      hosts: ENV.fetch('DB_HOSTS', nil).split(',')
    )

    connection = cluster.connect(keyspace)

    register('database.connection', connection)
  end
end
