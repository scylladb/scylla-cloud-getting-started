defmodule MediaPlayer.Actions do
  def cluster, do: MediaPlayer.Config.Database.start_link()

  def run_query(query) do
    case Xandra.Cluster.execute(cluster(), query) do
      {:ok, result} ->
        result

      {:error, error} ->
        IO.inspect(error)
    end
  end

  def run_query(query, params) do
    prepared = Xandra.Cluster.prepare!(cluster(), query)

    case Xandra.Cluster.execute(cluster(), prepared, params) do
      {:ok, result} ->
        result

      {:error, error} ->
        IO.inspect(error)
    end
  end
end
