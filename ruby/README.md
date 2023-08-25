# ScyllaDB Cloud Media Player Metrics

Project to store songs that you like to listen daily and keep track of them in a shape of a CLI!

## Prerequisites

* [Ruby]()
* [Cassandra related libraries](https://cassandra.apache.org/doc/latest/cassandra/getting_started/installing.html)

> Disclaimer: This gem require system wide dependencies with the cassandra client, so it's required to install on your system (or run the whole application under a docker image).

## Running the project

Clone the repository into your machine:

```sh
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/ruby
```

Install the project dependencies and run the project: 

```sh
bundle && ruby main.rb scylla yourpassword node-0,node-1,node-2
```

> Replace the variables with your cluster information

## Available Commands

Check which commands are currently available on this sample:

| Command  | Description |
|---|---|
| !new   | Add a new song to your liked songs list   |
| !delete  | Delete a specific song from your liked songs list   |
| !listen  | Creates a register of which song and when you listened to it  |
| !stress  | Retrieve all songs and create a 'stressing' loop to test a ScyllaDB Cloud Cluster |

## CQL Queries

All the CQL queries used on the project

```sql
CREATE KEYSPACE media_player
  WITH replication = {
    'class': 'NetworkTopologyStrategy',
    'replication_factor': '3'
  } AND durable_writes = true

CREATE TABLE media_player.playlist (
  id uuid,
  title text,
  album text,
  artist text,
  created_at timestamp,
  PRIMARY KEY (id, created_at)
) WITH CLUSTERING ORDER BY (created_at DESC);

CREATE TABLE media_player.song_counter (
  song_id uuid,
  times_played counter,
  PRIMARY KEY (song_id)
)

INSERT INTO media_player.playlist (id,title,artist,album,created_at) VALUES (now(),?,?,?,?);
SELECT * FROM media_player.playlist LIMIT 1;
UPDATE media_player.song_counter SET times_played = times_played + 1 WHERE song_id = ?

SELECT * FROM media_player.playlist;

DELETE FROM media_player.playlist WHERE id = ?
```
