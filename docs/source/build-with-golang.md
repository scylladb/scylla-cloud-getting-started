# Quick Start: Golang

In this tutorial we're gonna build a simple Media Player to store our songs and build playlists

## 1. Setup the Environment

### 1.1 Downloading golang dependencies:

If you don't have golang installed already on your machine, you can install from the following sources:

1. [Golang](https://go.dev/)

### 1.2 Starting the project

Now with the Golang installed, let's create a new project with the following command:

```sh
go mod init scylla-cloud-getting-started/golang
```

### 1.3 Setting the project dependencies

First we'll install the required package to connect to scyllaDB with the following command:

```sh
go get -u github.com/scylladb/gocqlx/v2
```

This package can be found at [github](https://github.com/scylladb/gocqlx)

## 2. Connecting to the Cluster

Make sure to get the right credentials on your [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) in the tab `Connect`.

```go
func main() {
    cluster := gocql.NewCluster("node-0.aws-sa-east-1.xxx.clusters.scylla.cloud", "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud","node-2.aws-sa-east-1.xxx.clusters.scylla.cloud")

    cluster.Authenticator = gocql.PasswordAuthenticator{Username: "Canhassi", Password: "password123"}
	cluster.PoolConfig.HostSelectionPolicy = gocql.DCAwareRoundRobinPolicy("AWS_US_EAST_1")

	session, err := gocqlx.WrapSession(cluster.CreateSession())

	if err != nil {
		panic("Connection fail")
	}
}
```

> If the connection got refused, check if your IP Address is added into allowed IPs.

## 3. Handling Queries

Using the `gocqlx` package you can instantiate a session and then run fully queries.

```go
type Song struct {
    Id string
    Title string
    Artist string
    Album string
    Created_at time.Time
}

func main() {
    song := Song{}
    
    q := session.Query("SELECT * FROM media_player.playlist", nil)

	if err := q.SelectRelease(&song); err != nil {
	    panic("error in exec query to list playlists: %w", err)
	}

    println(song)
}
```

### 3.1 Creating a Keyspace

The `keyspace` inside the ScyllaDB ecossystem can be interpreted as your `database` or `collection`.

On your connection boot, you don't need to provide it but you will use it later and also is able to create when you need.

```go
cluster := gocql.NewCluster("node-0.aws-sa-east-1.xxx.clusters.scylla.cloud", "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud","node-2.aws-sa-east-1.xxx.clusters.scylla.cloud")

cluster.Authenticator = gocql.PasswordAuthenticator{Username: "Canhassi", Password: "password123"}
cluster.PoolConfig.HostSelectionPolicy = gocql.DCAwareRoundRobinPolicy("AWS_US_EAST_1")

session, err := gocqlx.WrapSession(cluster.CreateSession())

if err != nil {
    panic("Connection fail")
}

session.Query("CREATE KEYSPACE IF NOT EXISTS media_player WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'}  AND durable_writes = true;", nil).Exec()
```

### 3.2 Creating a table

A table is used to store part or all the data of your app (depends on how you will build it). 
Remember to add your `keyspace` into your connection and let's create a table to store our liked songs.

```go
cluster := gocql.NewCluster("node-0.aws-sa-east-1.xxx.clusters.scylla.cloud", "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud","node-2.aws-sa-east-1.xxx.clusters.scylla.cloud")

cluster.Authenticator = gocql.PasswordAuthenticator{Username: "Canhassi", Password: "password123"}
cluster.PoolConfig.HostSelectionPolicy = gocql.DCAwareRoundRobinPolicy("AWS_US_EAST_1")

session, err := gocqlx.WrapSession(cluster.CreateSession())

if err != nil {
    panic("Connection fail")
}

session.Query("CREATE TABLE IF NOT EXISTS media_player.playlist (id uuid,title text,album text,artist text,created_at timestamp,PRIMARY KEY (id, created_at)) WITH CLUSTERING ORDER BY (created_at DESC)", nil).Exec()
```

### 3.3 Inserting data

Now that we have the keyspace and a table inside of it, we need to bring some good songs and populate it.

```go
type Song struct {
    Id string
    Title string
    Artist string
    Album string
    Created_at time.Time
}

song := Song{}

q := session.Query(
    `INSERT INTO media_player.playlist (id,title,artist,album,created_at) VALUES (now(),?,?,?,?)`,
    []string{":title", ":artist", ":album", ":created_at"}).
    BindMap(map[string]interface{} {
        ":title":      song.Title,
        ":artist":     song.Artist,
        ":album":      song.Album,
        ":created_at": time.Now(),
    })

err := q.Exec(); if err != nil {
    panic("error in exec query to insert a song in playlist %w", err)
}
```

### 3.4 Reading data

Since probably we added more than 3 songs into our database, let's list it into our terminal.

```go
type Song struct {
    Id string
    Title string
    Artist string
    Album string
    Created_at time.Time
}

func (s Song) String() string {
	return fmt.Sprintf("Id: %s\nTitle: %s\nArtist: %s\nAlbum: %s\nCreated At: %s\n", s.Id, s.Title, s.Artist, s.Album, s.Created_at)
}

song := Song{}

q := session.Query("SELECT * FROM media_player.playlist", nil)

if err := q.SelectRelease(&song); err != nil {
    panic("error in exec query to list playlists: %w", err)
}

println(song)
```

The result will look like:

```
Id: a1500b3b-5a38-11ee-97d6-4495929e9df0
Title: title teste
Artist: artist teste
Album: album teste
Created At: 2023-09-23 17:42:56.541 +0000 UTC
```

### 3.5 Updating data

Ok, almost there! Now we're going to learn about update but here's a disclaimer: 
> INSERT and UPDATES are not equals!

There's a myth in Scylla/Cassandra community that it's the same for the fact that you just need the `Partition Key` and `Clustering Key` (if you have one) and query it.

If you want to read more about it, [click here.](https://docs.scylladb.com/stable/using-scylla/cdc/cdc-basic-operations.html)

As we can see, the `UPDATE QUERY` takes two fields on `WHERE` (PK and CK). Check the snippet below: 

```go
q := session.Query(
    `UPDATE media_player.playlist SET 
        id = :id,
        title = :title,
        artist = :artist,
        album = :album,
        created_ad = :created_at
        WHERE id = :id`,
    []string{":id", ":title", ":artist", ":album", ":created_at"}).
    BindMap(map[string]interface{} {
        ":id":         "40450211-42cc-11ee-b14c-3da98b5024c0",
        ":title":      "CPFMGD",
        ":artist":     "Canhassi",
        ":album":      "Canhas desu",
        ":created_at": time.Now(),
    })

err := q.Exec(); if err != nil {
    panic("error in exec update query")
}
```

After updated, let's query for the ID and see the results:

```
scylla@cqlsh:media_player> select * from media_player.playlist where id = 40450211-42cc-11ee-b14c-3da98b5024c0;


 id                                   | created_at                      | album             | artist          | title
--------------------------------------+---------------------------------+-------------------+-----------------+---------------------------------
 40450211-42cc-11ee-b14c-3da98b5024c0 | 2023-09-16 18:22:56.397000+0000 |    Canhas desu    |     Canhassi    | CPFMGD

(1 rows)
```

It only "updated" the field `title`, `album` and `artist`(that is our Clustering Key) and since we didn't inputted the rest of the data, it will not be replicated as expected.

### 3.5 Deleting data

Let's understand what we can DELETE with this statement. There's the normal `DELETE` statement that focus on `ROWS` and other one that delete data only from `COLUMNS` and the syntax is very similar.

```sql 
-- Deletes a single row
DELETE FROM songs WHERE id = 40450211-42cc-11ee-b14c-3da98b5024c0;

-- Deletes a whole column
DELETE artist FROM songs WHERE id = 40450211-42cc-11ee-b14c-3da98b5024c0;
```

If you want to erase a specific column, you also should pass as parameter the `Clustering Key` and be very specific in which register you want to delete something. 
On the other hand, the "normal delete" just need the `Partition Key` to handle it. Just remember: if you use the statement "DELETE FROM keyspace.table_name" it will delete ALL the rows that you stored with that ID. 

```go
q := session.Query(`DELETE FROM media_player.playlist WHERE id = ?`,
    []string{":id"}).
    BindMap(map[string]interface{} {
        ":id": songToDelete.Id,
    })

err := q.Exec(); if err != nil {
    return fmt.Errorf("error to exec delete query %w", err)
}
```

## Conclusion

Yay! You now have the knowledge to use the basics of ScyllaDB with Golang.

If you thinks that something can be improved, please open an issue and let's make it happen!

Did you like the content? Dont forget to star the repo and follow us on socials.
