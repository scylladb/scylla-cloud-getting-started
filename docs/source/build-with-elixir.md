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
      {:decimal, "~> 1.0"},
      {:xandra, "~> 0.14"}
    ]
  end
```

- [Decimal](https://hexdocs.pm/decimal/readme.html): Arbitrary precision decimal arithmetic
- [Xandra](https://github.com/lexhide/xandra): Fast, simple, and robust Cassandra/ScyllaDB driver for Elixir

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

```
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