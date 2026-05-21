# ScyllaDB Cloud Media Player Metrics

Project to store songs that you like to listen daily and keep track of them in a shape of a CLI!

## Prerequisites

* [Node.js](https://nodejs.org/) >= 20
* [npm](https://www.npmjs.com/)

## Running the project

Clone the repository into your machine:

```sh
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/javascript
```

Install the project dependencies:

```sh
npm install
```

**Copy** `.env.example` to `.env` and fill in your cluster credentials from the [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) → **Connect** tab:

```sh
cp .env.example .env
```

Run the project:

```sh
node index.js
```

## Available Commands

Check which commands are currently available on this sample:

| Command   | Description                                                                     |
|-----------|---------------------------------------------------------------------------------|
| `!new`    | Add a new song to your liked songs list                                         |
| `!delete` | Delete a specific song from your liked songs list                               |
| `!listen` | Creates a register of which song you listened to and increments its play count  |
| `!stress` | Loop incrementing counters for all songs to stress-test a ScyllaDB Cloud cluster |

## CQL Queries

All the CQL queries used in the project:

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

INSERT INTO media_player.playlist (id, title, album, artist, created_at) VALUES (?, ?, ?, ?, ?);

SELECT id, title, album, artist, created_at FROM media_player.playlist;

UPDATE media_player.song_counter SET times_played = times_played + 1 WHERE song_id = ?;

DELETE FROM media_player.playlist WHERE id = ?;
```
