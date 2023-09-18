defmodule MediaPlayer.Config.Database do
  import Dotenv

  load()

  def start_link do
    options = [
      username: System.get_env("SCYLLADB_USERNAME"),
      password: System.get_env("SCYLLADB_PASSWORD")
    ]

    {:ok, cluster} =
      Xandra.Cluster.start_link(
        sync_connect: :infinity,
        authentication: {Xandra.Authenticator.Password, options},
        nodes:
          # Add the cluster connection urls separated by commas without spaces
          # Example: scylladb-node1,scylladb-node2,scylladb-node3
          System.get_env("SCYLLADB_NODE")
          |> String.split(",")
      )

    cluster
  end
end
