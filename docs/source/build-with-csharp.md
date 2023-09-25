# Quick Start: Csharp

In this tutorial we're gonna build a simple Media Player to store our songs and build playlists

## 1. Setup the Environment

### 1.1 Downloading .NET dependecies:

If you don't have csharp installed already on your machine, you can install from the following source:

1. [.NET main website](https://dotnet.microsoft.com/en-us/download/)

### 1.2 Starting the project

Now with the .NET installed, let's create a new project with the following command:

```sh
dotnet new console -n media_player && cd media_player
```

### 1.3 Setting the project dependencies

First we'll install the required package to connect to scyllaDB with the following command:

```sh
dotnet add package CassandraCSharpDriver
```

This package can be found at [github](https://github.com/datastax/csharp-driver/)

> Disclaimer: This package require system wide dependencies with the cassandra client, so it's required to install on your system (or run the whole application under a docker image). You can find the installation guide at: https://cassandra.apache.org/doc/latest/cassandra/getting_started/installing.html


## 2. Connecting to the Cluster

Make sure to get the right credentials on your [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) in the tab `Connect`.

```csharp
using Cassandra;

namespace Program;

public class Program
{
    static void Main(string[] args)
    {
        var cluster = Cluster.Builder()
                     .AddContactPoints("node-0.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-1.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-2.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .WithCredentials("scylla", "a-very-secure-password")
                     .Build();
    }
}

```

> If the connection got refused, check if your IP Address is added into allowed IPs.

## 3. Handling Queries

Using the `cassandra` package you can instantiate a session and then run fully queries.

```csharp
using Cassandra;

namespace Program;

public class Program
{
    static void Main(string[] args)
    {
        var session = cluster.Connect();
        var rs = session.Execute("SELECT address, port, connection_stage FROM system.clients LIMIT 5");
        foreach (var row in rs)
        {
            Console.Write($"IP -> {row["address"]}, Port -> {row["port"]}, CS -> {row["connection_stage"]}");
        }
    }
}

```

The output should look something like:

```
IP -> 172.17.0.1, Port -> 52830, CS -> READY
```

### 3.1 Creating a Keyspace

The `keyspace` inside the ScyllaDB ecossystem can be interpreted as your `database` or `collection`.

On your connection boot, you don't need to provide it but you will use it later and also is able to create when you need.

```csharp
using Cassandra;

namespace Program;

public class Program
{
    static void Main(string[] args)
    {
        var cluster = Cluster.Builder()
                     .AddContactPoints("node-0.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-1.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-2.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .WithCredentials("scylla", "a-very-secure-password")
                     .Build();

        string keyspace = "media_player";

        var session = cluster.Connect();
        var ps = session.Prepare("select keyspace_name from system_schema.keyspaces WHERE keyspace_name=?");

        var statement = ps.Bind(keyspace);

        var result = session.Execute(statement);

        var rows = result
                    .GetRows()
                    .ToList();

        if(rows.Count == 0)
        {
            StringBuilder sb = new();
            sb.Append($"CREATE KEYSPACE {keyspace} ");
            sb.Append("WITH replication = { 'class': 'NetworkTopologyStrategy', 'replication_factor': '3'}");
            sb.Append("AND durable_writes = true");
                
            session.Execute(sb.ToString());
            Console.WriteLine("Keyspace created!");
        }
        else
        {
            Console.WriteLine("Keyspace already created!");
        }

        // Reconnecting to the cluster with the correct keyspace
        session = cluster.Connect("media_player")
    }
}
```

### 3.2 Creating a table

A table is used to store part or all the data of your app (depends on how you will build it). 
Remember to add your `keyspace` into your connection and let's create a table to store our liked songs.

```csharp
using Cassandra;

namespace Program;

public class Program
{
    static void Main(string[] args)
    {
        var cluster = Cluster.Builder()
                      .AddContactPoints("node-0.aws-sa-east-1.xxx.clusters.scylla.cloud")
                      .AddContactPoints("node-1.aws-sa-east-1.xxx.clusters.scylla.cloud")
                      .AddContactPoints("node-2.aws-sa-east-1.xxx.clusters.scylla.cloud")
                      .WithCredentials("scylla", "a-very-secure-password")
                      .Build();

        string keyspace = "media_player";
        string table = "playlist";

        var session = cluster.Connect(keyspace);
        var ps = session.Prepare("select keyspace_name, table_name from system_schema.tables where keyspace_name = ? AND table_name = ?");

        var statement = ps.Bind(keyspace, table);

        var result = session.Execute(statement);

        var rows = result
                    .GetRows()
                    .ToList();

        if(rows.Count == 0)
        {
            StringBuilder sb = new();
            sb.Append($"CREATE TABLE {keyspace}.{table} (id uuid, title text, album text, artist text, created_at timestamp, PRIMARY KEY(id, created_at));");
            session.Execute(sb.ToString());
            Console.WriteLine("Table created!");
        }
        else
        {
            Console.WriteLine("Table already created!");
        }
    }
}
```

### 3.3 Inserting data

Now that we have the keyspace and a table inside of it, we need to bring some good songs and populate it. 

```csharp
using Cassandra;

namespace Program;

public class Program
{
    public record Song(string Title, string Album, string Artist, DateTime CreatedAt);
    static void Main(string[] args)
    {
        var cluster = Cluster.Builder()
                     .AddContactPoints("node-0.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-1.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-2.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .WithCredentials("scylla", "a-very-secure-password")
                     .Build();   

        string keyspace = "media_player";
        string table = "playlist";
        
        var session = cluster.Connect(keyspace);
        
        List<Song> songs = new()
        {
            new Song("Bohemian Rhapsody", "A night at the Opera", "Queen", DateTime.Now),
            new Song("Closer to the Edge", "This Is War", "Thirty Seconds to Mars", DateTime.Now),
            new Song("I Write Sins Not Tragedies", "A Fever You Can't Sweat Out", "Panic! at the Disco", DateTime.Now),
        };

        var ps = session.Prepare($"INSERT INTO {table} (id, title, album, artist, created_at) VALUES (?, ?, ?, ?, ?)");

        foreach(Song song in songs)
        {
            var uuid = Guid.NewGuid();
            var statement = ps.Bind(uuid,song.Title, song.Album, song.Artist, song.CreatedAt);
            session.Execute(statement);
        }
    }
}
```

### 3.4 Reading data

Since probably we added more than 3 songs into our database, let's list it into our terminal.

```csharp
using Cassandra;

namespace Program;

public class Program
{
    static void Main(string[] args)
    {
        var cluster = Cluster.Builder()
                     .AddContactPoints("node-0.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-1.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-2.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .WithCredentials("scylla", "a-very-secure-password")
                     .Build();   

        string keyspace = "media_player";
        string table = "playlist";
        
        var session = cluster.Connect(keyspace);
        
        var result = session.Execute($"SELECT id, title, album, artist FROM {table}");

        var rows = result
                    .GetRows()
                    .ToList();

        foreach(var row in rows)
        {
            Console.WriteLine($"ID: {row["id"]} | Song: {row["title"]} | Album: {row["album"]} | Artist: {row["artist"]}" );
        }
    }
}
```

The result will look like:

```
ID: 0d617157-57aa-48aa-98dc-f850b74e6aba | Song: Bohemian Rhapsody | Album: A night at the Opera | Artist: Queen
ID: 1b9a90fa-ec7a-4469-babb-f375fb8635cf | Song: Closer to the Edge | Album: This Is War | Artist: Thirty Seconds to Mars
ID: e59da413-d185-4065-9b7c-ae36f5203e90 | Song: I Write Sins Not Tragedies | Album: A Fever You Can't Sweat Out | Artist: Panic! at the Disco
```

### 3.5 Updating data

Ok, almost there! Now we're going to learn about update but here's a disclaimer: 
> INSERT and UPDATES are not equals!

There's a myth in Scylla/Cassandra community that it's the same for the fact that you just need the `Partition Key` and `Clustering Key` (if you have one) and query it.

If you want to read more about it, [click here.](https://docs.scylladb.com/stable/using-scylla/cdc/cdc-basic-operations.html)

As we can see, the `UPDATE QUERY` takes two fields on `WHERE` (PK and CK). Check the snippet below: 

```csharp
using Cassandra;
using System.Globalization;

namespace Program;

public class Program
{
    public record Song(string Title, string Album, string Artist, DateTime CreatedAt);

    static void Main(string[] args)
    {
        var cluster = Cluster.Builder()
                     .AddContactPoints("node-0.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-1.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-2.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .WithCredentials("scylla", "a-very-secure-password")
                     .Build();   

        string keyspace = "media_player";
        string table = "playlist";
        
        var session = cluster.Connect(keyspace);
        
        Song song = new(
            "Bohemian Rhapsody Updated",
            "A night at the Opera Updated",
            "Queen Updated",
            DateTime.ParseExact("2023-09-16 18:22:56.397000+0000", "yyyy-MM-dd HH:mm:ss.ffffffzzz", CultureInfo.InvariantCulture));

        var ps = session.Prepare($"UPDATE {keyspace}.{table} SET title = ?, album = ?, artist = ? where id = ? and created_at = ?");
        var statement = ps.Bind(song.Title, song.Album, song.Artist, new Guid("0d617157-57aa-48aa-98dc-f850b74e6aba"), song.CreatedAt); 

        session.Execute(statement);
        
        Console.WriteLine("Song Updated");
    }
}
```

After updated, let's query for the ID and see the results:

```
scylla@cqlsh:media_player> select * from media_player.playlist where id = 0d617157-57aa-48aa-98dc-f850b74e6aba;


 id                                   | created_at                      | album             | artist          | title
--------------------------------------+---------------------------------+-------------------+-----------------+---------------------------------
 0d617157-57aa-48aa-98dc-f850b74e6aba | 2023-09-16 18:22:56.397000+0000 | 2023-09-16 18:22:56.397000+0000 | Queen Updated | Bohemian Rhapsody Updated

(1 rows)
```

It only "updated" the field `title`, `album` and `artist`(that is our Clustering Key) and since we didn't inputted the rest of the data, it will not be replicated as expected.

### 3.5 Deleting data

Let's understand what we can DELETE with this statement. There's the normal `DELETE` statement that focus on `ROWS` and other one that delete data only from `COLUMNS` and the syntax is very similar.

```sql 
-- Deletes a single row
DELETE FROM songs WHERE id = 0d617157-57aa-48aa-98dc-f850b74e6aba;

-- Deletes a whole column
DELETE artist FROM songs WHERE id = 0d617157-57aa-48aa-98dc-f850b74e6aba;
```

If you want to erase a specific column, you also should pass as parameter the `Clustering Key` and be very specific in which register you want to delete something. 
On the other hand, the "normal delete" just need the `Partition Key` to handle it. Just remember: if you use the statement "DELETE FROM keyspace.table_name" it will delete ALL the rows that you stored with that ID. 

```csharp
using Cassandra;
using System.Globalization;

namespace Program;

public class Program
{
    public record Song(string Title, string Album, string Artist, DateTime CreatedAt);

    static void Main(string[] args)
    {
        var cluster = Cluster.Builder()
                     .AddContactPoints("node-0.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-1.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .AddContactPoints("node-2.aws-sa-east-1.xxx.clusters.scylla.cloud")
                     .WithCredentials("scylla", "a-very-secure-password")
                     .Build();   

        string keyspace = "media_player";
        string table = "playlist";
        
        var session = cluster.Connect(keyspace);
        
        Song song = new(
            "Bohemian Rhapsody Updated",
            "A night at the Opera Updated",
            "Queen Updated",
            DateTime.ParseExact("2023-09-16 18:22:56.397000+0000", "yyyy-MM-dd HH:mm:ss.ffffffzzz", CultureInfo.InvariantCulture));

        var ps = session.Prepare($"DELETE FROM {keyspace}.{table} where id = ? and created_at = ?");
        var statement = ps.Bind(new Guid("0d617157-57aa-48aa-98dc-f850b74e6aba"), song.CreatedAt); 

        session.Execute(statement);
        
        Console.WriteLine("Song deleted!");
    }
}
```

## Conclusion

Yay! You now have the knowledge to use the basics of ScyllaDB with Csharp.

If you thinks that something can be improved, please open an issue and let's make it happen!

Did you like the content? Dont forget to star the repo and follow us on socials.
