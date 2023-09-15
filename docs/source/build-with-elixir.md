# Quick Start: Elixir

## 1. Setup the Environment

### 1.1 Downloading Elixir and dependencies:

If you don't have Elixir and Erlang installed already on your machine, you can install from two possible sources:

1. [Elixir main website](https://elixir-lang.org/install.html)
2. [asdf](https://asdf-vm.com/guide/getting-started.html)
> NOTE: After installing asdf correctly make sure to install [Erlang](https://github.com/asdf-vm/asdf-erlang) first and then [Elixir](https://github.com/asdf-vm/asdf-elixir).

### 1.2 Starting the project

Now with Elixir properly installed you can create a project using [mix](https://elixir-lang.org/getting-started/mix-otp/introduction-to-mix.html). To create our project just run:

```sh
mix new media_player
```

### 1.3 Setting the project dependencies

Let's do a quick change into our `mix.exs` and add our project dependencies.

```exs
defp deps do
  [
    {:dotenv, "~> 3.0"},
    {:decimal, "~> 1.0"},
    {:xandra, "~> 0.14"},
    {:elixir_uuid, "~> 1.2"}
  ]
end
```
- [Dotenv](https://hexdocs.pm/dotenv/Dotenv.html): A port of dotenv to Elixir 
- [Decimal](https://hexdocs.pm/decimal/readme.html): Arbitrary precision decimal arithmetic
- [Xandra](https://github.com/lexhide/xandra): Fast, simple, and robust Cassandra/ScyllaDB driver for Elixir
- [Elixir UUID](https://hexdocs.pm/uuid/readme.html): UUID generator and utilities for Elixir 

To carry out modifications, use the module already created in `lib/media_player.ex`, as this is where we are going to carry out some modifications.

> NOTE: In this tutorial we are going to prepare an interactive project so that you can perform tests using Elixir's interactive shell. On every code update, don't forget to restart Elixir's interactive shell to perform the recompilation.

## 2. Connecting to the Cluster

Make sure to get the right credentials on your [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) in the tab `Connect`.

```ex
def start_link do
  options = [username: "scylla", password: "a-very-secure-password"]

  {:ok, cluster} =
    Xandra.Cluster.start_link(
      sync_connect: :infinity,
      authentication: {Xandra.Authenticator.Password, options},
      nodes: [
        "node-0.aws-sa-east-1.xxx.clusters.scylla.cloud",
        "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud",
        "node-2.aws-sa-east-1.xxx.clusters.scylla.cloud"
      ],
      pool_size: 10
    )
  
  cluster
end
```

When starting a cluster notice that there is an option with the name `sync_connect` being informed. This option is used to inform that we are dealing with an asynchronous connection, saying to wait the necessary time (`:infinity`) to make the complete connection of the cluster.

To test our connection let's initialize Elixir's interactive shell:

```sh
iex -S mix
```

Then you will see a screen waiting for some input, with in the bottom left corner a message that looks like `iex(1)>` (this means we are ready to test our first module). To test it now, let's run:

```ex
MediaPlayer.start_link
```

A process will be started, so the return should be nothing other than something like `#PID<0.230.0>`. Don't worry, in the next topic we'll run our first query and see a real result. This function will be used every time we need to start a connection link with our cluster.

> If the connection got refused, check if your IP Address is added into allowed IPs.

## 3. Handling Queries

With `Xandra` you can run queries and save their returns in maps, making it possible to parse this information and manipulate it as you decide. First of all, let's create a function that will simply execute queries, receiving information from the cluster and the query to be executed as parameters. If the return is `:ok`, it means that the query executed successfully, so we return it. If the return is `:error`, it means that we had an error, so let's inspect it. An important detail is for the address, which instead of bringing a simple text brings a tuple with four integers.

```ex
def run_query(cluster, query) do
  case Xandra.Cluster.execute(cluster, query) do
    {:ok, result} ->
      result

    {:error, error} ->
      IO.inspect(error)
  end
end

def handling_queries do
  statement = "SELECT address, port, connection_stage FROM system.clients LIMIT 5;"

  run_query(start_link(), statement)
  |> Enum.each(fn %{
                    "address" => address,
                    "connection_stage" => connection_stage,
                    "port" => port
                  } ->
    # `address` is a tuple of 4 integers
    address_formated =
      address
      |> Tuple.to_list()
      |> Enum.map(&Integer.to_string/1)
      |> Enum.join(".")

    IO.puts("IP -> #{address_formated}, Port -> #{port}, CS -> #{connection_stage}")
  end)
end
```

The output should look something like:

```
IP -> 123.123.123.69, Port -> 61667, CS -> READY
IP -> 123.123.123.69, Port -> 62377, CS -> AUTHENTICATING
IP -> 123.123.123.69, Port -> 63221, CS -> AUTHENTICATING
IP -> 123.123.123.69, Port -> 65225, CS -> READY
```

### 3.1 Creating a Keyspace

The `keyspace` inside the ScyllaDB ecossystem can be interpreted as your `database` or `collection`.

On your connection boot, you don't need to provide it but you will use it later and also is able to create when you need.

```ex
def keyspace_exists?(keyspace_name) do
  cluster = start_link()

  # In this case I won't use the `run_query` function because I want to 
  # show the possibility of using maps to bind its parameters.
  %Xandra.Page{} =
    page =
    Xandra.Cluster.run(cluster, fn conn ->
      prepared =
        Xandra.prepare!(
          conn,
          "SELECT * FROM system_schema.keyspaces WHERE keyspace_name = :keyspace_name;"
        )

      Xandra.execute!(conn, prepared, %{"keyspace_name" => keyspace_name})
    end)

  Enum.to_list(page) != []
end

def create_keyspace(keyspace_name) do
  case keyspace_exists?(keyspace_name) do
    true ->
      IO.puts("Keyspace already exists")

    false ->
      cluster = start_link()

      query = "CREATE KEYSPACE IF NOT EXISTS #{keyspace_name}
                WITH REPLICATION = {
                  'class': 'NetworkTopologyStrategy',
                  'replication_factor': '3'
                }
                AND durable_writes = true;"

      run_query(cluster, query)

      IO.puts("Keyspace created")
  end
end
```

To test run your interactive shell again with `iex -S mix` and then run:

```ex
iex(1)> MediaPlayer.create_keyspace("media_player")
```

Done! Now your keyspace is officially created!

### 3.2 Creating a table

A table is used to store part or all the data of your app (depends on how you will build it). Remember to add your `keyspace` into your connection and let's create a table to store our liked songs.

```ex
def table_exists?(keyspace_name, table_name) do
  cluster = start_link()

  %Xandra.Page{} =
    page =
    Xandra.Cluster.run(cluster, fn conn ->
      prepared =
        Xandra.prepare!(
          conn,
          "SELECT keyspace_name, table_name FROM system_schema.tables WHERE keyspace_name = :keyspace_name AND table_name = :table_name;"
        )

      Xandra.execute!(conn, prepared, %{
        "keyspace_name" => keyspace_name,
        "table_name" => table_name
      })
    end)

  Enum.to_list(page) != []
end

def create_table(keyspace_name, table_name) do
  case table_exists?(keyspace_name, table_name) do
    true ->
      IO.puts("Table already exists")

    false ->
      cluster = start_link()

      query = "CREATE TABLE IF NOT EXISTS #{keyspace_name}.#{table_name} (
                id uuid,
                title text,
                album text,
                artist text,
                created_at timestamp,
                PRIMARY KEY (id, created_at)
              );"

      run_query(cluster, query)

      IO.puts("Table created")
  end
end
```

We added a check if the table exists, passing the keyspace name and the table name as parameters, working in the same way as the keyspace check. To test your table creation, open the interactive shell again and run:

```ex
iex(1)> MediaPlayer.create_table("media_player", "playlist")
```

Done! Now your playlist table is officially created!

### 3.3 Inserting data

Now that we have the keyspace and a table inside of it, we need to bring some good songs and populate it.

First of all, let's add a dependency to our `mix.exs` to work with UUID, then just run `mix deps.get` to update the dependencies!

```exs
{:elixir_uuid, "~> 1.2"}
```

To execute our query, let's create another `run_query` function that will receive three parameters (the cluster, the query and the other parameters) to prepare our execution. In Elixir, we can have functions with the same name but different number of parameters, in which it will be understood that they are different functions.

```ex
def run_query(cluster, query, params) do
  prepared = Xandra.Cluster.prepare!(cluster, query)

  case Xandra.Cluster.execute(cluster, prepared, params) do
    {:ok, result} ->
      result

    {:error, error} ->
      IO.inspect(error)
  end
end

def insert_songs(keyspace, table) do
  cluster = start_link()

  song_list = [
    %{
      id: UUID.uuid4(),
      title: "Getaway Car",
      album: "Reputation",
      artist: "Taylor Swift",
      created_at: DateTime.utc_now()
    },
    %{
      id: UUID.uuid4(),
      title: "Still Into You",
      album: "Paramore",
      artist: "Paramore",
      created_at: DateTime.utc_now()
    },
    %{
      id: UUID.uuid4(),
      title: "Stolen Dance",
      album: "Sadnecessary",
      artist: "Milky Chance",
      created_at: DateTime.utc_now()
    }
  ]

  Enum.each(song_list, fn %{
                            id: id,
                            title: title,
                            album: album,
                            artist: artist,
                            created_at: created_at
                          } ->
    query =
      "INSERT INTO #{keyspace}.#{table} (id, title, album, artist, created_at) VALUES (?, ?, ?, ?, ?);"

    run_query(cluster, query, [id, title, album, artist, created_at])
  end)
end
```

The need to create a function was to properly prepare our query with our arguments. To test just run the interactive shell again with `iex -S mix` and run:

```ex
iex(1)> MediaPlayer.insert_songs("media_player", "playlist")
```

### 3.4 Reading data

Since probably we added more than 3 songs into our database, let's list it into our terminal.

```ex
def read_data(keyspace, table) do
  cluster = start_link()

  query = "SELECT id, title, album, artist, created_at FROM #{keyspace}.#{table};"

  run_query(cluster, query)
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
```

To test just run the interactive shell again with `iex -S mix` and run:

```ex
iex(1)> MediaPlayer.read_data("media_player", "playlist")
```

The result will look like:

```
ID: 09679e47-0898-40fd-b114-52b27de5a21c | Title: Stolen Dance | Album: Sadnecessary | Artist: Milky Chance | Created At: 2023-09-07 22:26:56.798Z
ID: 56fac772-dc54-4518-86df-2a628a2a45f6 | Title: Still Into You | Album: Paramore | Artist: Paramore | Created At: 2023-09-07 22:26:56.798Z
ID: 11bbeed9-c9a8-45cc-9842-c60483b4cb67 | Title: Getaway Car | Album: Reputation | Artist: Taylor Swift | Created At: 2023-09-07 22:26:56.798Z
```

### 3.5 Updating data

Ok, almost there! Now we're going to learn about update but here's a disclaimer: 
> INSERT and UPDATES are not equals!

There's a myth in Scylla/Cassandra community that it's the same for the fact that you just need the `Partition Key` and `Clustering Key` (if you have one) and query it.

If you want to read more about it, [click here.](https://docs.scylladb.com/stable/using-scylla/cdc/cdc-basic-operations.html)

As we can see, the `UPDATE QUERY` takes two fields on `WHERE` (PK and CK). Check the snippet below: 

```ex
def update_data(keyspace, table) do
  cluster = start_link()

  query =
    "UPDATE #{keyspace}.#{table} SET title = ?, album = ?, artist = ? WHERE id = ? AND created_at = ?;"

  {:ok, date_formated, _} = DateTime.from_iso8601("2023-09-07 22:26:56.798Z")

  run_query(cluster, query, [
    "Getaway Car UPDATED",
    "Reputation",
    "Taylor Swift",
    "11bbeed9-c9a8-45cc-9842-c60483b4cb67",
    date_formated
  ])
end
```

Note that we had to convert the saved date format to iso8601. To test just run the interactive shell again with `iex -S mix` and run:

```ex
iex(1)> MediaPlayer.update_data("media_player", "playlist")
```

So to check if it's been updated:

```ex
iex(2)> MediaPlayer.read_data("media_player", "playlist")
```

The result will look like:

```
ID: 09679e47-0898-40fd-b114-52b27de5a21c | Title: Stolen Dance | Album: Sadnecessary | Artist: Milky Chance | Created At: 2023-09-07 22:26:56.798Z
ID: 56fac772-dc54-4518-86df-2a628a2a45f6 | Title: Still Into You | Album: Paramore | Artist: Paramore | Created At: 2023-09-07 22:26:56.798Z
ID: 11bbeed9-c9a8-45cc-9842-c60483b4cb67 | Title: Getaway Car UPDATED | Album: Reputation | Artist: Taylor Swift | Created At: 2023-09-07 22:26:56.798Z
```

### 3.5 Deleting data

Let's understand what we can DELETE with this statement. There's the normal `DELETE` statement that focus on `ROWS` and other one that delete data only from `COLUMNS` and the syntax is very similar.

```sql 
-- Deletes a single row
DELETE FROM songs WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;

-- Deletes a whole column
DELETE artist FROM songs WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;
```

If you want to erase a specific column, you also should pass as parameter the `Clustering Key` and be very specific in which register you want to delete something. On the other hand, the "normal delete" just need the `Partition Key` to handle it. Just remember: if you use the statement "DELETE FROM keyspace.table_name" it will delete ALL the rows that you stored with that ID. 

```ex
def delete_data(keyspace, table) do
  cluster = start_link()

  query = "DELETE FROM #{keyspace}.#{table} WHERE id = ? AND created_at = ?;"

  {:ok, date_formated, _} = DateTime.from_iso8601("2023-09-07 22:26:56.798Z")

  run_query(cluster, query, [
    "11bbeed9-c9a8-45cc-9842-c60483b4cb67",
    date_formated
  ])
end
```

To test just run the interactive shell again with `iex -S mix` and run:

```ex
iex(1)> MediaPlayer.delete_data("media_player", "playlist")
```

So to check if it's been updated:

```ex
iex(2)> MediaPlayer.read_data("media_player", "playlist")
```

The result will look like:

```
ID: 09679e47-0898-40fd-b114-52b27de5a21c | Title: Stolen Dance | Album: Sadnecessary | Artist: Milky Chance | Created At: 2023-09-07 22:26:56.798Z
ID: 56fac772-dc54-4518-86df-2a628a2a45f6 | Title: Still Into You | Album: Paramore | Artist: Paramore | Created At: 2023-09-07 22:26:56.798Z
```

## Conclusion

Yay! You now have the knowledge to use the basics of ScyllaDB with Elixir.

If you thinks that something can be improved, please open an issue and let's make it happen!

Did you like the content? Dont forget to star the repo and follow us on socials.