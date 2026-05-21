# Quick start: Rust (Crablang)

In this tutorial you'll build a simple Media Player to store your songs and manage a playlist via the command line.

## 1. Setup the Environment

Let's download Rust and the dependencies needed for this project. 

### 1.1 Downloading Rust and Dependencies

If you don't have Rust installed yet, run the command below — it installs Rust and Cargo.
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 1.2 Starting the project

Now with Rust and Cargo installed, create a new project:

```sh
cargo new media_player
cd media_player
```

### 1.3 Setting the project dependencies

Update `Cargo.toml` with the project dependencies:

```toml
[package]
name = "media_player"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.72"
chrono = "0.4.26"
clap = { version = "4.3.21", features = ["derive"] }
futures = "0.3.28"
rand = "0.8.5"
scylla = { version = "1.0", features = ["chrono-04"] }
serde = { version = "1.0.174", features = ["derive", "serde_derive"] }

[dependencies.tokio]
version = "1"
features = ["full"]

[dependencies.uuid]
version = "1.4.1"
features = [
    "v4",
    "fast-rng",
    "macro-diagnostics",
]
```

* [Scylla](https://crates.io/crates/scylla): ScyllaDB Rust driver. `chrono-04` enables serialization of `chrono` types.
* [Clap](https://crates.io/crates/clap): CLI argument parsing.
* [Uuid](https://crates.io/crates/uuid): UUID generation.
* [Tokio](https://crates.io/crates/tokio): Async runtime for Rust.
* [Anyhow](https://crates.io/crates/anyhow): Idiomatic error handling.
* [Chrono](https://crates.io/crates/chrono): DateTime/Timestamp handling.
* [Futures](https://crates.io/crates/futures): Utilities for working with Rust futures.

## 2. Connecting to the Cluster

Make sure to get the right credentials on your [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) in the tab `Connect`.

The app accepts connection details as CLI arguments. Define the argument struct with [clap](https://crates.io/crates/clap):

```rust
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author, version, about)]
pub struct ConnectionDetails {
    /// Scylla Cloud Node URL's (one or more, space- or comma-separated)
    #[arg(num_args = 1.., value_parser, value_delimiter = ',')]
    pub nodes: Vec<String>,

    /// Cluster Username
    #[arg(short, long)]
    username: String,

    /// Cluster Password
    #[arg(short, long)]
    password: String,
}
```

Then build the session in `database.rs`:

```rust
use anyhow::Context;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::time::Duration;

pub async fn new_session(config: &ConnectionDetails) -> Result<Session, anyhow::Error> {
    let nodes = config
        .nodes
        .iter()
        .filter(|i| !i.is_empty())
        .collect::<Vec<_>>();

    let session: Session = SessionBuilder::new()
        .known_nodes(nodes)
        .connection_timeout(Duration::from_secs(5))
        .user(config.username.to_string(), config.password.to_string())
        .build()
        .await
        .context("Connection Refused. Check your credentials and your IP linked on the ScyllaDB Cloud.")?;

    Ok(session)
}
```

> If the connection is refused, check that your IP address is added to the allowed IPs list in your ScyllaDB Cloud dashboard.

Run the app with:

```sh
cargo run -- --username scylla --password your-cluster-password \
  node-0.aws-sa-east-1.xxx.clusters.scylla.cloud \
  node-1.aws-sa-east-1.xxx.clusters.scylla.cloud \
  node-2.aws-sa-east-1.xxx.clusters.scylla.cloud
```

## 3. Handling Queries

The ScyllaDB Rust driver supports prepared statements (recommended for repeated queries) and unpaged/iter queries.  
Use `session.prepare()` to prepare a statement once, then `execute_unpaged` or `execute_iter` to run it.

```rust
use futures::TryStreamExt;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::net::IpAddr;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let session: Session = SessionBuilder::new()
        .known_nodes(&[
            "node-0.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-2.aws-sa-east-1.xxx.clusters.scylla.cloud",
        ])
        .connection_timeout(Duration::from_secs(30))
        .user("scylla", "your-awesome-password")
        .build()
        .await
        .expect("connection refused");

    let query = "SELECT address, port, connection_stage FROM system.clients LIMIT 5";

    session
        .query_iter(query, &[])
        .await?
        .rows_stream::<(IpAddr, i32, String)>()?
        .try_for_each(|row| async move {
            println!("IP -> {}, Port -> {}, CS -> {}", row.0, row.1, row.2);
            Ok(())
        })
        .await?;

    Ok(())
}
```

Output should look like: 
```
IP -> 123.123.123.69, Port -> 61667, CS -> READY
IP -> 123.123.123.69, Port -> 62377, CS -> AUTHENTICATING
```



### 3.1 Creating a Keyspace

The `keyspace` in ScyllaDB can be interpreted as your `database`. You can check whether it already exists using the driver's cluster metadata API and create it if needed.

```rust
use anyhow::Result;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let keyspace = String::from("prod_media_player");

    let session: Session = SessionBuilder::new()
        .known_nodes(&[
            "node-0.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-2.aws-sa-east-1.xxx.clusters.scylla.cloud",
        ])
        .connection_timeout(Duration::from_secs(5))
        .user("scylla", "your-password")
        .build()
        .await
        .unwrap();

    // Use the driver's metadata API to check if the keyspace already exists.
    // Normally you'd use CREATE KEYSPACE IF NOT EXISTS, but this showcases the metadata API.
    let has_keyspace = session
        .get_cluster_state()
        .get_keyspace(&keyspace)
        .is_some();

    if !has_keyspace {
        let create_ks = format!(
            "CREATE KEYSPACE {} WITH REPLICATION = {{'class': 'NetworkTopologyStrategy', 'replicationFactor': '3'}};",
            keyspace
        );
        session.query_unpaged(create_ks, &[]).await?;
        println!("Keyspace {} created!", &keyspace)
    } else {
        println!("Keyspace {} already exists!", &keyspace)
    }

    Ok(())
}
```

### 3.2 Creating a Table

A table stores your application data. Here we create the `songs` table in the `prod_media_player` keyspace.

```rust
use anyhow::Result;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::time::Duration;

static KEYSPACE: &str = "prod_media_player";
static TABLE: &str = "songs";

#[tokio::main]
async fn main() -> Result<()> {
    let session: Session = SessionBuilder::new()
        .known_nodes(&[
            "node-0.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-2.aws-sa-east-1.xxx.clusters.scylla.cloud",
        ])
        .connection_timeout(Duration::from_secs(30))
        .user("scylla", "****")
        .build()
        .await
        .expect("connection refused");

    // Use the driver's metadata API to check if the table already exists.
    let has_table = session
        .get_cluster_state()
        .get_keyspace(KEYSPACE)
        .and_then(|ks| ks.tables.get(TABLE))
        .is_some();

    if !has_table {
        let create_table = format!(
            "CREATE TABLE {}.{} (
                id uuid,
                title text,
                album text,
                artist text,
                created_at timestamp,
                PRIMARY KEY (id, created_at)
            )",
            KEYSPACE, TABLE
        );

        session.query_unpaged(create_table, &[]).await?;
        println!("Table {} created!", TABLE)
    } else {
        println!("Table {} already exists!", TABLE)
    }

    Ok(())
}
```

### 3.3 Inserting data

Now that we have the keyspace and table, let's populate it with songs. We use prepared statements for efficiency.

```rust
use anyhow::Result;
use chrono::Utc;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::str::FromStr;
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let keyspace = "prod_media_player";
    let table = "songs";

    let session: Session = SessionBuilder::new()
        .known_nodes(&[
            "node-0.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-2.aws-sa-east-1.xxx.clusters.scylla.cloud",
        ])
        .connection_timeout(Duration::from_secs(5))
        .user("scylla", "your-password")
        .build()
        .await
        .unwrap();

    let now = Utc::now();

    let song_list = vec![
        (
            Uuid::new_v4(),
            "Stairway to Heaven",
            "Led Zeppelin IV",
            "Led Zeppelin",
            now,
        ),
        (
            Uuid::from_str("d754f8d5-e037-4898-af75-44587b9cc424").unwrap(),
            "Glimpse of Us",
            "Smithereens",
            "Joji",
            now,
        ),
        (Uuid::new_v4(), "Vegas", "From Movie ELVIS", "Doja Cat", now),
    ];

    let insert_query = format!(
        "INSERT INTO {}.{} (id, title, album, artist, created_at) VALUES (?,?,?,?,?)",
        keyspace, table
    );

    let prepared = session.prepare(insert_query).await?;

    for song in &song_list {
        session.execute_unpaged(&prepared, song).await?;
        println!("Inserted: {}", song.1);
    }

    Ok(())
}
```

### 3.4 Reading data

List songs from the database using `execute_iter` with a prepared statement:

```rust
use anyhow::Result;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let session: Session = SessionBuilder::new()
        .known_nodes(&[
            "node-0.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-2.aws-sa-east-1.xxx.clusters.scylla.cloud",
        ])
        .connection_timeout(Duration::from_secs(5))
        .user("scylla", "your-awesome-password")
        .build()
        .await
        .unwrap();

    let prepared = session
        .prepare(
            "SELECT id, title, album, artist, created_at FROM prod_media_player.songs LIMIT 10",
        )
        .await?;

    session
        .execute_iter(prepared, &[])
        .await?
        .rows_stream::<(Uuid, String, String, String, DateTime<Utc>)>()?
        .try_for_each(|row| async move {
            println!("ID: {} | Song: {} | Album: {} | Created At: {}", row.0, row.1, row.2, row.4);
            Ok(())
        })
        .await?;

    Ok(())
}
```

The result will look like this:

```
ID: d754f8d5-e037-4898-af75-44587b9cc424 | Song: Glimpse of Us | Album: Smithereens | Created At: 2023-08-09 02:11:55 UTC
ID: 72f940de-1c8b-46ce-a09d-8c52e66fa21f | Song: Stairway to Heaven | Album: Led Zeppelin IV | Created At: 2023-08-09 02:11:55 UTC
```

### 3.5 Deleting Data

Delete a row by its partition key (`id`). Since `id` is the partition key, this removes all rows with that UUID:

```rust
use anyhow::Result;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::str::FromStr;
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let session: Session = SessionBuilder::new()
        .known_nodes(&[
            "node-0.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-2.aws-sa-east-1.xxx.clusters.scylla.cloud",
        ])
        .connection_timeout(Duration::from_secs(5))
        .user("scylla", "your-awesome-password")
        .build()
        .await
        .unwrap();

    let song_id = Uuid::from_str("d754f8d5-e037-4898-af75-44587b9cc424").unwrap();

    let prepared_query = session
        .prepare("DELETE FROM prod_media_player.songs WHERE id = ?")
        .await?;

    session
        .execute_unpaged(&prepared_query, (song_id,))
        .await?;

    println!("Song deleted!");

    Ok(())
}
```

## 4. Running the Full Application

Clone the repository and run the complete Media Player app:

```sh
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/rust
cargo run -- --username scylla --password your-cluster-password \
  node-0.aws-sa-east-1.xxx.clusters.scylla.cloud \
  node-1.aws-sa-east-1.xxx.clusters.scylla.cloud \
  node-2.aws-sa-east-1.xxx.clusters.scylla.cloud
```

Once started, you'll see a prompt with available commands:

```
------------------------------------
Here some possibilities
  !add    - add new song
  !list   - list all songs
  !delete - delete a specific song
  !stress - stress testing with mocked data
  !q      - quit
------------------------------------
Type any command: 
```

## Conclusion

You now have the knowledge to use the basics of ScyllaDB with Rust.

If you think something can be improved, please open an issue and let's make it happen!

Did you like the content? Don't forget to star the repo and follow us on socials.
