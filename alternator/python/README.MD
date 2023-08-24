# ScyllaDB Cloud Media Player Metrics

Project to store songs that you like to listen daily and keep track of them using the command line.

## Prerequisites

* [Python 3.7+](https://www.python.org/downloads/)
* [Virtualenv](https://virtualenv.pypa.io/en/latest/installation.html)

## Running the project

Clone the repository:

```sh 
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/alternator/python
```

Start a new virtual environment for the project:

```sh
virtualenv env
source env/bin/activate
```

Install the driver: 

```sh
pip install boto3
```

Go to `environment.py` and set up your ScyllaDB Cloud Cluster credentials and run:

```sh
python cli.py
```

## Available Commands

| Command  | Description |
|---|---|
| !new   | Add a new song to your liked songs list   |
| !delete  | Delete a specific song from your liked songs list   |
| !listen  | Creates a register of which song and when you listened to it  |

## CQL Queries

All the CQL queries used in the project:

```sql
CREATE KEYSPACE prod_media_player
    WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'}
    AND durable_writes = true;

CREATE TABLE songs (
    id uuid,
    title text,
    album text,
    artist text,
    release_year int,
    created_at timestamp,
    PRIMARY KEY (id, created_at)
)

CREATE TABLE recently_played_songs (
    song_id uuid,
    listened_at timestamp,
    PRIMARY KEY (song_id, created_at)
)

CREATE TABLE played_songs_counter (
    song_id uuid,
    times_played count,
    PRIMARY KEY (song_id)
)

SELECT keyspace_name FROM system_schema.keyspaces WHERE keyspace_name = ?
SELECT keyspace_name,table_name FROM system_schema.tables WHERE keyspace_name = ? AND table_name = ?

SELECT * FROM songs
INSERT INTO recently_played_songs (song_id, listened_at) VALUES (?, ?)
UPDATE played_songs_counter SET times_played = times_played + 1 WHERE song_id = ?
DELETE FROM songs WHERE id = ?

```