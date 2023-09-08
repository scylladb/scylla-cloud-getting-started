defmodule MediaPlayer.Config.Connection do
  import Dotenv

  load()

  def keyspace() do
    System.get_env("SCYLLADB_KEYSPACE")
  end

  def table() do
    System.get_env("SCYLLADB_TABLE")
  end
end
