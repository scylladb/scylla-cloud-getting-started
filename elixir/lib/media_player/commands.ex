defmodule MediaPlayer.Commands do
  use Task

  alias MediaPlayer.Actions, as: Actions
  alias MediaPlayer.Config.Connection, as: Connection

  defp keyspace, do: Connection.keyspace()
  defp table, do: Connection.table()

  def add_from(title, album, artist, created) do
    query =
      "INSERT INTO #{keyspace()}.#{table()} (id, title, album, artist, created_at) VALUES (?, ?, ?, ?, ?);"

    {:ok, created, _} = DateTime.from_iso8601(created <> "T00:00:00Z")

    Actions.run_query(query, [UUID.uuid4(), title, album, artist, created])

    IO.puts("Song added!")
  end

  def add() do
    title = IO.gets("Enter the title of the song: ") |> String.trim()
    album = IO.gets("Enter the album of the song: ") |> String.trim()
    artist = IO.gets("Enter the artist of the song: ") |> String.trim()

    created =
      IO.gets("Enter the date the song was created (YYYY-MM-DD): ")
      |> String.trim()

    add_from(title, album, artist, created)
  end

  def list do
    query = "SELECT id, title, album, artist, created_at FROM #{keyspace()}.#{table()};"

    Actions.run_query(query)
    |> Enum.each(fn %{
                      "id" => id,
                      "title" => title,
                      "album" => album,
                      "artist" => artist,
                      "created_at" => created_at
                    } ->
      IO.puts(
        "ID: #{id} | Title: #{title} | Album: #{album} | Artist: #{artist} | Created At: #{created_at}"
      )
    end)
  end

  def delete() do
    query = "SELECT id, title, album, artist, created_at FROM #{keyspace()}.#{table()};"

    songs =
      Actions.run_query(query)
      |> Enum.with_index(fn %{
                              "id" => id,
                              "title" => title,
                              "album" => album,
                              "artist" => artist,
                              "created_at" => created_at
                            },
                            index ->
        IO.puts(
          "Index: #{index + 1} | Title: #{title} | Album: #{album} | Artist: #{artist} | Created At: #{created_at}"
        )

        %{id: id, title: title, album: album, artist: artist, created_at: created_at}
      end)

    {input, _} = IO.gets("Enter the index of the song you want to delete: ") |> Integer.parse()

    case Enum.at(songs, input - 1) do
      %{} = song ->
        query = "DELETE FROM #{keyspace()}.#{table()} WHERE id = ? AND created_at = ?;"

        Actions.run_query(query, [song.id, song.created_at])

        IO.puts("Song deleted!")

      nil ->
        IO.puts("Invalid index.")
    end
  end

  defp generate_stress_query(some_id) do
    current_date = Date.to_string(Date.utc_today())

    "INSERT INTO #{keyspace()}.#{table()} (
      id, title, album, artist, created_at
    ) VALUES (
      #{UUID.uuid4()},
      'Test Song #{some_id}',
      'Test Artist #{some_id}',
      'Test Album #{some_id}',
      '#{current_date}'
    );"
  end

  def stress do
    start = Time.utc_now()
    cluster = MediaPlayer.Config.Database.start_link()

    # Simple stress test
    1..100_000
    |> Task.async_stream(
      fn id ->
        IO.puts("[#{id}] Adding seed")
        Xandra.Cluster.execute(cluster, generate_stress_query(id))
      end,
      max_concurrency: 500
    )
    |> Enum.to_list()

    IO.puts("Time taken: #{Time.diff(Time.utc_now(), start, :second)} seconds")
  end
end
