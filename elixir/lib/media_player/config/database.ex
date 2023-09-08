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
        nodes: [
          System.get_env("SCYLLADB_NODE_1")
          # If you want to use more nodes, uncomment the following lines:
          # System.get_env("SCYLLADB_NODE_2"),
          # System.get_env("SCYLLADB_NODE_3")
        ]
      )

    cluster
  end
end
