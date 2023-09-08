defmodule MediaPlayer do
  @moduledoc """
  Documentation for `MediaPlayer`.
  """

  @doc """
  Hello from ScyllaDB!

  This is a simple application example using Elixir with ScyllaDB!
  The project consists of using `Xandra`.

  To run the project, you need to have a ScyllaDB cluster running.
  You can use ScyllaDB Cloud, ScyllaDB on Docker or ScyllaDB on Kubernetes.
  You can find more information about ScyllaDB on https://www.scylladb.com/

  To run the project, you need to have Elixir installed.
  You can find more information about Elixir on https://elixir-lang.org/

  To run the project, you need to have the following environment variables:
  - SCYLLADB_USERNAME
  - SCYLLADB_PASSWORD
  - SCYLLADB_NODE_1
  - SCYLLADB_NODE_2
  - SCYLLADB_NODE_3

  You can find more information about environment variables on
  https://elixir-lang.org/getting-started/mix-otp/config-and-releases.html#environment-configuration
  """
  alias MediaPlayer.Commands, as: Commands

  def loop do
    IO.puts("--------------------------------------")
    IO.puts("Type any command: ")
    command = IO.gets("") |> String.trim()

    case command do
      "!add" ->
        Commands.add()
        loop()

      "!list" ->
        Commands.list()
        loop()

      "!delete" ->
        Commands.delete()
        loop()

      "!stress" ->
        Commands.stress()
        loop()

      "exit" ->
        IO.puts("Bye bye!")
        :ok

      _ ->
        IO.puts("Command not found!")
        loop()
    end
  end

  def start(_, _) do
    run()
    {:ok, self()}
  end

  def run do
    IO.puts("--------------------------------------")
    IO.puts("- ScyllaDB Cloud Elixir Media Player -")
    IO.puts("-      Leave a star on the repo      -")
    IO.puts("--------------------------------------")
    IO.puts("Here some possibilities")
    IO.puts("  !add - add new song")
    IO.puts("  !list - list all songs")
    IO.puts("  !delete - delete a specific song")
    IO.puts("  !stress - stress testing with mocked data")
    IO.puts("--------------------------------------")

    loop()
  end
end
