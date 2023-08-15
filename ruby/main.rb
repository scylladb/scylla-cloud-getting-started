# frozen_string_literal: true

require 'cassandra'

cluster = Cassandra.cluster(
  username: 'scylla',
  password: 'a-password',
  hosts: [
    'node1.amazonaaws',
    'node2.amazonaws',
    'node3.amazonaws'
  ]
)

session  = cluster.connect

future = session.execute_async('SELECT address, port, connection_stage FROM system.clients LIMIT 5')
future.on_success do |rows|
  rows.each do |row|
    puts "IP -> #{row['address']}, Port -> #{row['port']}, CS -> #{row['connection_stage']}"
  end
end

future.join
