# Quick Start: Rust (Crablang)

In this tutorial we're gonna build a simple Media Player to store our songs and build playlists

## 1. Setup the Enviroment

Let's download Rust and the dependencies needed for this project. 

### 1.1 Downloading Rust and Dependencies

If you don't have rust installed in your machine yet, run the command below and it will install Rust and some other helpful tools (such as Cargo).
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 1.2 Starting the project

Now with the Rust and Cargo installed, just create a new project using this command:

```sh
cargo new media_player
```

### 1.3 Setting the project dependencies

Let's do a quick change into our `cargo.toml` and add our project dependencies. 

```toml
[package]
name = "media_player"
version = "0.1.0"
edition = "2021"

[dependencies]
scylla = "0.8.0"
uuid = {version = "0.8", features = ["v4"]}
tokio = { version = "1.17.0", features = ["full"] }
anyhow = "1.0.70"
chrono = "0.4.24"
```

* [Scylla](https://crates.io/crates/scylla): using the latest driver release
* [Uuid](https://crates.io/crates/uuid): help us to create Uuids in our project
* [Tokio](https://crates.io/crates/tokio): Async calls in Rust.
* [Anyhow](https://crates.io/crates/anyhow): Idiomatic Error Handling 
* [Chrono](https://crates.io/crates/chrono): DateTime/Timestamp Handling

## 2. Connecting to the Cluster

Make sure to get the right credentials on your [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) in the tab `Connect`.

```rust
use anyhow::Result;
use scylla::{Session, SessionBuilder};
use std::time::Duration;
#[tokio::main]
async fn main() -> Result<()> {

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
        .unwrap();

    Ok(())

}
```

> If the connection got refused, check if you IP Address is added into allowed ips.

## 3. Handling Queries

At Rust driver you can use the function inside your cluster connection called `query()` and build the query you want to execute inside your database/keyspace.

```rust
use anyhow::Result;
use scylla::{IntoTypedRows, Session, SessionBuilder, FromRow};
use std::net::IpAddr;
use std::time::Duration;


#[tokio::main]
async fn main() -> Result<()> {

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
   // Simpler way
   if let Some(rows) = session.query(query, &[]).await?.rows {
        for row in rows.into_typed::<(IpAddr, i32, String)>() {
            let row = row?;
            println!("IP -> {}, Port -> {}, CS -> {}", row.0, row.1, row.2);
        }
    }
    
    // Complex, but cool way
    session
        .query(query, &[])
        .await?
        .rows
        .map(|row| {
            row.into_typed::<(IpAddr, i32, String)>()
                .filter(|row| row.is_ok())
                .map(|row| row.unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap()
        .iter()
        .for_each(|row| println!("IP -> {}, Port -> {}, CS -> {}", row.0, row.1, row.2));
    Ok(())
}
```

Output should look like: 
```
IP -> 123.123.123.69, Port -> 61667, CS -> READY
IP -> 123.123.123.69, Port -> 62377, CS -> AUTHENTICATING
IP -> 123.123.123.69, Port -> 63221, CS -> AUTHENTICATING
IP -> 123.123.123.69, Port -> 65225, CS -> READY
```



### 3.1 Creating a Keyspace

The `keyspace` inside the ScyllaDB ecossystem can be interpreted as your `database` or `collection`.

On your connection boot, you don't need to provide it but you will use it later and also is able to create when you need.

```rust
use anyhow::Result;
use scylla::{Session, SessionBuilder};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let keyspace = String::from("media_player");

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

    // Verify if the Keyspace already exists in your Cluster
    let validate_keyspace_query = session
        .prepare("select keyspace_name from system_schema.keyspaces WHERE keyspace_name=?")
        .await?;

    let has_keyspace = session
        .execute(&validate_keyspace_query, (&keyspace,))
        .await?
        .rows_num()
        .unwrap();

    if has_keyspace == 0 {
        let new_keyspace_query = format!(
            "
            CREATE KEYSPACE {} 
                WITH replication = {{
                    'class': 'NetworkTopologyStrategy',
                     'replication_factor': '3'
                }}
                AND durable_writes = true
        ",
            keyspace
        );

        session.query(new_keyspace_query, &[]).await?;
        println!("Keyspace {} created!", &keyspace)
    } else {
        println!("Keyspace {} already created!", &keyspace)
    }

    session.use_keyspace(keyspace, false).await?;

    Ok(())
}
```

> After that you probably will need to re-create your connection poiting which `keyspace` you want to use.

### 3.2 Creating a Table

A table is used to store part or all the data of your app (depends on how you will build it). 
Remember to add your `keyspace` into your connection and let's create a table to store our liked songs.

```rust
use anyhow::Result;
use scylla::{Session, SessionBuilder};
use std::time::Duration;

static KEYSPACE: &str = "media_player_rust";  
static TABLE: &str = "playlist";
  
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
  
    // Verify if the table already exists in the specific Keyspace inside your Cluster
    let validate_table_query = session
        .prepare("
            select keyspace_name, table_name from system_schema.tables where keyspace_name = ? AND table_name = ?
        ")
        .await?;

    let has_table = session
        .execute(&validate_table_query, (&keyspace, &table))
        .await?
        .rows_num()
        .unwrap();

    if has_table == 0 {
        let new_keyspace_query = format!(
            "CREATE TABLE {}.{} (
                id uuid,
                title text,
                album text,
                artist text,
                created_at timestamp,
                PRIMARY KEY (id, updated_at)
            )",
            &keyspace, &table
        );

        session.query(new_keyspace_query, &[]).await?;
        println!("Table {} created!", &table)
    } else {
        println!("Table {} already created!", &table)
    }
  
    Ok(())
}
```

### 3.3 Inserting data

Now that we have the keyspace and a table inside of it, we need to bring some good songs and populate it. 

```rust
use anyhow::Result;
use chrono::{Duration, Utc};
use scylla::frame::value::Timestamp;
use scylla::{Session, SessionBuilder};
use std::{str::FromStr, time::Duration as ConnectionDuration};
use uuid::{self, Uuid};

async fn main() -> Result<()> {
    let keyspace = String::from("media_player");
    let table = String::from("songs");

    let session: Session = SessionBuilder::new()
        .known_nodes(&[
            "node-0.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud",
            "node-2.aws-sa-east-1.xxx.clusters.scylla.cloud",
        ])
        .connection_timeout(ConnectionDuration::from_secs(5))
        .user("scylla", "your-password")
        .build()
        .await
        .unwrap();

    session.use_keyspace(keyspace, false).await?;

    let song_list = vec![
        (
            Uuid::new_v4(),
            "Stairway to Heaven",
            "Led Zeppelin IV",
            "Led Zeppelin",
            Timestamp(Duration::seconds(Utc::now().timestamp())),
        ),
        (
            Uuid::from_str("d754f8d5-e037-4898-af75-44587b9cc424").unwrap(),
            "Glimpse of Us",
            "Smithereens",
            "Joji",
            Timestamp(Duration::seconds(Utc::now().timestamp())),
        ),
        (
            Uuid::new_v4(),
            "Vegas",
            "From Movie ELVIS",
            "Doja Cat",
            Timestamp(Duration::seconds(Utc::now().timestamp())),
        ),
    ];

    let insert_query = format!(
        "INSERT INTO {} (id,title,album,artist,created_at) VALUES (?,?,?,?,?)",
        table
    );

    let prepared = session.prepare(insert_query).await?;

    for song in song_list {
        session.execute(&prepared, song).await?;
        println!("Inserting Track: {}", song.1.to_string());
    }

    Ok(()) 
}
```

### 3.3 Reading data

Since probably we added more than 3 songs into our database, let's list it into our terminal.

```rust
use anyhow::Result;
use chrono::{Duration, Utc, DateTime};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;
use scylla::{Session, SessionBuilder, IntoTypedRows};
use std::{str::FromStr, time::Duration as ConnectionDuration};
use uuid::{self, Uuid};

#[tokio::main]

async fn main() -> Result<()> {
    let session: Session = SessionBuilder::new()
        .known_nodes(&[
            "your-node-1.aws-sa-east-1.1.clusters.scylla.cloud",
            "your-node-2.aws-sa-east-1.2.clusters.scylla.cloud",
            "your-node-3.aws-sa-east-1.3.clusters.scylla.cloud",
        ])
        .connection_timeout(ConnectionDuration::from_secs(5))
        .user("scylla", "your-awesome-password")
        .build()
        .await
        .unwrap();

    session.use_keyspace("media_player", false).await?;

    session.query("SELECT id, title, album, artist, created_at FROM songs", &[])
        .await?
        .rows
        .map(|row| {
            row.into_typed::<(Uuid, String, String, String, Duration)>()
                .map(|row| row.unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap()
        .iter()
        .for_each(|row| {
            println!(
                "Song: {} - Album: {} - Created At: {}", 
                row.1, row.2, row.4
            )
        });

    Ok(())
}
```

The result will look like this:

```
Song: Vegas - Album: From Movie ELVIS - Created At: P19578DT6810S
Song: Glimpse of Us - Album: Smithereens - Created At: P19578DT6810S
Song: Stairway to Heaven - Album: Led Zeppelin IV - Created At: P19578DT6810S
```

> Remeber to decode your Uuid if needed using the function `.toString()`

### 3.4 Updating Data

Ok, almost there! Now we're going to learn about update but here's a disclaimer: 
> INSERT and UPDATES are not equals!

There's a myth in Scylla/Cassandra community that it's the same for the fact that you just need the `Partition Key` and `Clustering Key` (if you have one) and query it.

If you want to read more about it, [click here.](https://docs.scylladb.com/stable/using-scylla/cdc/cdc-basic-operations.html)

As we can see, the `UPDATE QUERY` takes two fields on `WHERE` (PK and CK). Check the snippet below: 

```rust
use anyhow::Result;
use chrono::{Duration, Utc, DateTime};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;
use scylla::{Session, SessionBuilder, IntoTypedRows};
use std::{str::FromStr, time::Duration as ConnectionDuration};
use uuid::{self, Uuid};

async fn main() -> Result<()> {
    let session: Session = SessionBuilder::new()
        .known_nodes(&[
            "node-0.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
            "node-1.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
            "node-2.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
        ])
        .connection_timeout(ConnectionDuration::from_secs(5))
        .user("scylla", "your-password")
        .build()
        .await
        .unwrap();

    let song_to_update = (
        "Glimpse of Us",
        "2022 em uma música",
        "Inutilismo",
        Uuid::from_str("d754f8d5-e037-4898-af75-44587b9cc424").unwrap(),
        Timestamp(Duration::seconds(1691547115))
    );

    

    session.use_keyspace("media_player", false).await?;

    let prepared_query = session.prepare(
        "UPDATE songs set title = ?, album = ?, artist = ? where id = ? and created_at = ?"
    ).await?;
        
    session.execute(&prepared_query, song_to_update).await?;
    

    session.query("SELECT id, title, album, artist, created_at FROM songs WHERE id = ?", (song_to_update.3,))
        .await?
        .rows
        .map(|row| {
            row.into_typed::<(Uuid, String, String, String, Duration)>()
                .map(|row| row.unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap()
        .iter()
        .for_each(|row| {
            println!(
                "ID: {} -  Song: {} - Album: {} - Created At: {}", 
                row.0, row.1, row.2, row.4
            )
        });

    Ok(())
}
```
After inserted, let's query for the ID and see the results:

```
scylla@cqlsh:media_player> select * from songs where id = d754f8d5-e037-4898-af75-44587b9cc424;

 id                                   | created_at                      | album              | artist     | title
--------------------------------------+---------------------------------+--------------------+------------+---------------
 d754f8d5-e037-4898-af75-44587b9cc424 | 2023-08-09 02:11:55.000000+0000 | 2022 em uma música | Inutilismo | Glimpse of Us

(1 rows)
```

It only "updated" the field `title` and `updated_at` (that is our Clustering Key) and since we didn't inputted the rest of the data, it will not be replicated as expected.


### 3.5 Deleting Data

Let's understand what we can DELETE with this statement. There's the normal `DELETE` statement that focus on `ROWS` and other one that delete data only from `COLUMNS` and the syntax is very similar.

```sql 
// Deletes a single row
DELETE FROM songs WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;

// Deletes a whole column
DELETE artist FROM songs WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;
```

If you want to erase a specific column, you also should pass as parameter the `Clustering Key` and be very specific in which register you want to delete something. 
On the other hand, the "normal delete" just need the `Partition Key` to handle it. Just remember: if you use the statement "DELETE FROM keyspace.table_name" it will delete ALL the rows that you stored with that ID. 

```rust
use anyhow::Result;
use chrono::{Duration, Utc, DateTime};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;
use scylla::{Session, SessionBuilder, IntoTypedRows};
use std::{str::FromStr, time::Duration as ConnectionDuration};
use uuid::{self, Uuid};

#[tokio::main]

async fn main() -> Result<()> {
    let session: Session = SessionBuilder::new()
        .known_nodes(&[
            "node-0.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
            "node-1.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
            "node-2.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
        ])
        .connection_timeout(ConnectionDuration::from_secs(5))
        .user("scylla", "your-awesome-password")
        .build()
        .await
        .unwrap();

    let song_to_delete = (
        "Glimpse of Us",
        "2022 em uma música",
        "Inutilismo",
        Uuid::from_str("d754f8d5-e037-4898-af75-44587b9cc424").unwrap(),
        Timestamp(Duration::seconds(1691547115))
    );

    

    session.use_keyspace("media_player", false).await?;

    let prepared_query = session.prepare(
        "DELETE FROM songs where id = ? and created_at = ?"
    ).await?;
        
    session.execute(&prepared_query, (song_to_delete.3, song_to_delete.4,)).await?;
    println!("Song deleted!");

    Ok(())
}
```

## Conclusion

Yay! You now have the knowledge to use the basics of ScyllaDB with Rust.

If you thinks that something can be improved, please open an issue and let's make it happen!

Did you like the content? Dont forget to star the repo and follow us on socials.