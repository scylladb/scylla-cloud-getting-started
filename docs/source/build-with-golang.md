# Quick Start: Golang

In this tutorial we're going to build a simple Media Player to store our songs and build playlists.

## 1. Setup the Environment

### 1.1 Downloading Golang

If you don't have Golang installed already on your machine, you can install it from the following sources:

1. [Golang](https://go.dev/)

### 1.2 Cloning the project

Clone the repository and navigate to the Go example:

```sh
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/go
```

### 1.3 Configuring credentials

Copy `.env.example` to `.env` and fill in your ScyllaDB Cloud credentials:

```sh
cp .env.example .env
```

Edit `.env` with your cluster details:

```sh
NODES=node-0.aws-us-east-1.<cluster-id>.clusters.scylla.cloud
CLUSTER_USERNAME=scylla
CLUSTER_PASSWORD=your-password
# Datacenter name from the ScyllaDB Cloud dashboard (Connect tab), e.g. AWS_US_EAST_1
CLUSTER_REGION=AWS_US_EAST_1
MIGRATE_PATH=./internal/database/migrations/migrate.cql
```

> You can find the node hostnames and datacenter name in the [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) under the **Connect** tab.

### 1.4 Installing dependencies and running the project

```sh
go run ./cmd/...
```

## 2. Connecting to the Cluster

The connection is set up in `internal/database/connection.go`. It reads credentials from environment variables and configures a DC-aware load balancing policy for optimal performance:

```go
func Connect() (*gocqlx.Session, error) {
    nodes := os.Getenv("NODES")
    username := os.Getenv("CLUSTER_USERNAME")
    password := os.Getenv("CLUSTER_PASSWORD")
    region := os.Getenv("CLUSTER_REGION")

    hosts := strings.Split(nodes, ",")

    cluster := gocql.NewCluster(hosts...)

    cluster.Authenticator = gocql.PasswordAuthenticator{Username: username, Password: password}
    cluster.PoolConfig.HostSelectionPolicy = gocql.TokenAwareHostPolicy(gocql.DCAwareRoundRobinPolicy(region))

    session, err := gocqlx.WrapSession(cluster.CreateSession())
    if err != nil {
        return nil, err
    }

    return &session, nil
}
```

> If the connection is refused, check that your IP address is added to the allowed IPs in the ScyllaDB Cloud dashboard.

## 3. Handling Queries

Using the `gocqlx` package you can instantiate a session and run CQL queries.

```go
type Song struct {
    Id         string
    Title      string
    Artist     string
    Album      string
    Created_at time.Time
}

func (s Song) String() string {
    return fmt.Sprintf("Id: %s\nTitle: %s\nArtist: %s\nAlbum: %s\nCreated At: %s\n",
        s.Id, s.Title, s.Artist, s.Album, s.Created_at)
}
```

### 3.1 Creating a Keyspace

The `keyspace` in ScyllaDB is equivalent to a database or schema. The migration file at `internal/database/migrations/migrate.cql` creates it automatically on startup:

```cql
CREATE KEYSPACE IF NOT EXISTS media_player
  WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'}
  AND durable_writes = true;
```

### 3.2 Creating a table

A table stores the data for your app. The migration also creates the playlist table:

```sql
CREATE TABLE IF NOT EXISTS media_player.playlist (
    id uuid,
    title text,
    album text,
    artist text,
    created_at timestamp,
    PRIMARY KEY (id, created_at)
) WITH CLUSTERING ORDER BY (created_at DESC);
```

### 3.3 Inserting data

Now that we have the keyspace and a table, we can add songs to the playlist:

```go
func (c *SongController) Insert(song *database.Song) error {
    q := c.Session.Query(
        `INSERT INTO media_player.playlist (id, title, artist, album, created_at) VALUES (now(), ?, ?, ?, ?)`,
        []string{":title", ":artist", ":album", ":created_at"}).
        BindMap(map[string]interface{}{
            ":title":      song.Title,
            ":artist":     song.Artist,
            ":album":      song.Album,
            ":created_at": time.Now(),
        })

    if err := q.Exec(); err != nil {
        return fmt.Errorf("error in exec query to insert a song in playlist %w", err)
    }

    return nil
}
```

Use the `!add` command in the CLI to add a song interactively.

### 3.4 Reading data

List all songs stored in the playlist:

```go
func (c *SongController) List() ([]database.Song, error) {
    songs := []database.Song{}

    q := c.Session.Query("SELECT * FROM media_player.playlist", nil)

    if err := q.SelectRelease(&songs); err != nil {
        return songs, fmt.Errorf("error in exec query to list playlists: %w", err)
    }

    return songs, nil
}
```

Use the `!list` command in the CLI to display all songs. Each entry will look like:

```
Id: a1500b3b-5a38-11ee-97d6-4495929e9df0
Title: My Favourite Song
Artist: Some Artist
Album: Great Album
Created At: 2023-09-23 17:42:56.541 +0000 UTC
```

### 3.5 Deleting data

Delete a row by its partition key:

```go
func (c *SongController) Delete() error {
    songs, err := c.List()
    if err != nil {
        return err
    }

    index, err := c.selectSongToDelete(songs)
    if err != nil {
        return err
    }

    if index >= 0 && index < len(songs) {
        songToDelete := songs[index]

        q := c.Session.Query(`DELETE FROM media_player.playlist WHERE id = ?`,
            []string{":id"}).
            BindMap(map[string]interface{}{
                ":id": songToDelete.Id,
            })

        if err := q.Exec(); err != nil {
            return fmt.Errorf("error to exec delete query %w", err)
        }
    }

    return nil
}
```

Use the `!delete` command in the CLI to select and delete a song interactively.

### 3.6 Stress testing

The `!stress` command inserts 100,000 records concurrently to test cluster throughput:

```go
func (c *StressController) Stress() error {
    fmt.Println("Inserting 100,000 records into the database...")

    start := time.Now()

    var wg sync.WaitGroup
    sem := make(chan bool, 550)

    for i := 0; i < 100_000; i++ {
        sem <- true
        wg.Add(1)
        go func() {
            defer func() {
                <-sem
                wg.Done()
            }()

            q := c.Session.Query(
                `INSERT INTO media_player.playlist (id, title, artist, album, created_at) VALUES (now(), ?, ?, ?, ?)`,
                []string{":title", ":artist", ":album", ":created_at"}).
                BindMap(map[string]interface{}{
                    ":title":      "title teste",
                    ":artist":     "artist teste",
                    ":album":      "album teste",
                    ":created_at": time.Now(),
                })

            if err := q.Exec(); err != nil {
                fmt.Println(err.Error())
            }
        }()
    }

    wg.Wait()
    fmt.Println("Time taken:", time.Since(start))

    return nil
}
```

## Conclusion

You now have the knowledge to use the basics of ScyllaDB with Golang.

If you think something can be improved, please open an issue and let's make it happen!

Did you like the content? Don't forget to star the repo and follow us on socials.

