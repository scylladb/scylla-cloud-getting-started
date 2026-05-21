# ScyllaDB Cloud Media Player Metrics - Rust

Project to store songs that you like to listen daily and keep track of them in a shape of a CLI!

![GYJ01DE-1.png](https://i.imgur.com/GYJ01DE.png)

## Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)


## Running the project

Clone the repository into your machine:

```sh 
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/rust
```


Install the project dependencies and run the project: 

```sh
cargo run -- --username scylla --password your-cluster-password \
  node-0.aws-sa-east-1.1.clusters.scylla.cloud \
  node-1.aws-sa-east-1.1.clusters.scylla.cloud \
  node-2.aws-sa-east-1.1.clusters.scylla.cloud
```

> Replace the variables with your cluster information. You can also provide a single node — the driver will discover the rest automatically.


## Available Commands

Check which commands are currently available on this sample:

| Command  | Description |
|---|---|
| !add     | Add a new song to your liked songs list   |
| !list    | List all songs in your liked songs list   |
| !delete  | Delete a specific song from your liked songs list   |
| !stress  | Retrieve all songs and create a 'stressing' loop to test a ScyllaDB Cloud Cluster |
| !q       | Quit the application |

## CQL Queries

All the CQL queries used on the project

```sql
CREATE KEYSPACE prod_media_player WITH REPLICATION = {'class': 'NetworkTopologyStrategy', 'replicationFactor': '3'};

CREATE TABLE prod_media_player.songs (
    id uuid,
    title text,
    album text,
    artist text,
    created_at timestamp,
    PRIMARY KEY (id, created_at)
);

SELECT keyspace_name FROM system_schema.keyspaces WHERE keyspace_name = ?
SELECT keyspace_name,table_name FROM system_schema.tables WHERE keyspace_name = ? AND table_name = ?

SELECT id, title, album, artist, created_at FROM prod_media_player.songs LIMIT 10
INSERT INTO prod_media_player.songs (id, title, artist, album, created_at) VALUES (?, ?, ?, ?, ?)
DELETE FROM prod_media_player.songs WHERE id = ?

```