# frozen_string_literal: true

# cassandra-driver 3.2.5 was written against Cassandra 3.x and does not
# recognise ScyllaDB-specific CQL types such as `vector`.  When the driver
# fetches cluster schema on connect it raises IncompleteTypeError for any
# type it doesn't know.  This patch makes those unknown types fall back to
# a generic blob type so the connection succeeds.
module CassandraScyllaDBTypePatch
  def parse(*args)
    super
  rescue ::Cassandra::Cluster::Schema::CQLTypeParser::IncompleteTypeError
    ::Cassandra::Types.blob
  end
end

::Cassandra::Cluster::Schema::CQLTypeParser.prepend(CassandraScyllaDBTypePatch)
