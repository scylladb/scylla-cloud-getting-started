# ScyllaDB Cloud Media Player Metrics

Project to store songs that you like to listen to daily and keep track of them via a CLI!

## Prerequisites

* [Java 11+](https://adoptium.net/)
* [Apache Maven 3.6+](https://maven.apache.org/)

## Running the project

Clone the repository into your machine:

```sh
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/java
```

**rename** your `.env.example` to `.env` and fill in your cluster details:

```sh
cp .env.example .env
```

Build the fat JAR and run the app:

```sh
mvn package -q
java -jar target/app.jar
```

> Make sure your machine's IP address is allowed in your [ScyllaDB Cloud cluster's](https://cloud.scylladb.com/clusters) firewall rules.

## Available Commands

Check which commands are currently available on this sample:

| Command   | Description |
|-----------|---|
| !new      | Add a new song to your liked songs list |
| !delete   | Delete a specific song from your liked songs list |
| !listen   | Creates a register of which song and when you listened to it |
| !stress   | Retrieve all songs and create a 'stressing' loop to test a ScyllaDB Cloud Cluster |

## CQL Queries

All the CQL queries used on the project

```sql
CREATE KEYSPACE IF NOT EXISTS media_player
  WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'};

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

INSERT INTO media_player.playlist (id, title, artist, album, created_at) VALUES (?, ?, ?, ?, ?);

SELECT id, title, album, artist FROM media_player.playlist PER PARTITION LIMIT 1 LIMIT 100;

UPDATE media_player.song_counter SET times_played = times_played + 1 WHERE song_id = ?;

DELETE FROM media_player.playlist WHERE id = ?;
```
