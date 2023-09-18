# ScyllaDB Cloud Media Player Metrics - Elixir

Project to store songs that you like to listen daily and keep track of them in a shape of a CLI!

## Prerequisites

* [Elixir](https://elixir-lang.org/)

## Running the project

Clone the repository into your machine:

```sh 
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/elixir
```

Install the project dependencies and run the project:

```sh
mix deps.get && mix run
```

> Replace the environment variables with your cluster information

## Available Commands

Check which commands are currently available on this sample:

| Command  | Description |
|---|---|
| !add   | Add a new song to your liked songs list   |
| !delete  | Delete a specific song from your liked songs list   |
| !list  | Creates a register of which song and when you listened to it  |
| !stress  | Retrieve all songs and create a 'stressing' loop to test a ScyllaDB Cloud Cluster |

## CQL Queries

All the CQL queries used on the project

```sql
CREATE KEYSPACE prod_media_player
    WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'}
    AND durable_writes = true;

CREATE TABLE prod_media_player.songs (
    id uuid,
    title text,
    album text,
    artist text,
    created_at timestamp,
    PRIMARY KEY (id, created_at)
);

CREATE TABLE prod_media_player.song_counter (
    song_id uuid,
    times_played counter,
    PRIMARY KEY (song_id)
);

SELECT keyspace_name FROM system_schema.keyspaces WHERE keyspace_name = ?
SELECT keyspace_name,table_name FROM system_schema.tables WHERE keyspace_name = ? AND table_name = ?

SELECT * FROM songs
INSERT INTO recently_played_songs (song_id, listened_at) VALUES (?, ?)
UPDATE played_songs_counter SET times_played = times_played + 1 WHERE song_id = ?
DELETE FROM songs WHERE id = ?

```