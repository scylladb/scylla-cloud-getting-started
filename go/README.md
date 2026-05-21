# ScyllaDB Cloud Media Player

Project to store songs that you like to listen daily and keep track of them in a shape of a CLI!

## Prerequisites

* [Go](https://go.dev/)

## Running the project

Clone the repository into your machine:

```sh
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/go
```

Copy `.env.example` to `.env` and fill in your credentials:

```sh
cp .env.example .env
```

Set the following variables in `.env`:

| Variable | Description |
|---|---|
| `NODES` | Comma-separated list of ScyllaDB node hostnames |
| `CLUSTER_USERNAME` | ScyllaDB username |
| `CLUSTER_PASSWORD` | ScyllaDB password |
| `CLUSTER_REGION` | ScyllaDB datacenter name (e.g., `AWS_US_EAST_1`) — find it in the ScyllaDB Cloud dashboard under **Connect** |
| `MIGRATE_PATH` | Path to the migration file (default: `./internal/database/migrations/migrate.cql`) |

Install dependencies and run the project:

```bash
go run ./cmd/...
```

## Available Commands

| Command | Description |
|---|---|
| `!add` | Add a new song to your playlist |
| `!list` | List all songs in your playlist |
| `!delete` | Delete a specific song from your playlist |
| `!stress` | Stress test by inserting 100,000 records |
| `!q` | Quit the console |

## CQL Queries

All the CQL queries used in the project:

```sql
CREATE KEYSPACE IF NOT EXISTS media_player
  WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'}
  AND durable_writes = true;

CREATE TABLE IF NOT EXISTS media_player.playlist (
  id uuid,
  title text,
  album text,
  artist text,
  created_at timestamp,
  PRIMARY KEY (id, created_at)
) WITH CLUSTERING ORDER BY (created_at DESC);

CREATE TABLE IF NOT EXISTS media_player.song_counter (
  song_id uuid,
  times_played counter,
  PRIMARY KEY (song_id)
);

INSERT INTO media_player.playlist (id, title, artist, album, created_at) VALUES (now(), ?, ?, ?, ?);

SELECT * FROM media_player.playlist;

DELETE FROM media_player.playlist WHERE id = ?;
```
