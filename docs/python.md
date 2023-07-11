# Quick Start: Python

In this tutorial you'll build a Media Player to store your songs and build playlists.

## 1. Getting the Driver

### Prerequisites:
* [Python 3.7+](https://www.python.org/downloads/)
* [Virtualenv](https://virtualenv.pypa.io/en/latest/installation.html)

Create a new virtual environment and activate it:
```bash
virtualenv env
source env/bin/activate
```

Install the [Python ScyllaDB Driver](https://pypi.org/project/scylla-driver/).

```bash
pip install scylla-driver
```

## 2. Connecting to the Cluster

Get your database credentials from your [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) in the tab `Connect`.

```python


cluster = Cluster(
    contact_points=[
        "your-node-url-1.clusters.scylla.cloud",
        "your-node-url-2.clusters.scylla.cloud", 
        "your-node-url-3.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='your-awesome-password')
)
```

> If the connection gets refused, check if your IP Address is added to the list of allowed IP addresses.

## 3. Handling Queries

With the Python driver, you can use the function inside your cluster connection called `execute(query)` and build the query you want to execute inside your database/keyspace. You also can use the `execute_async()` to asyncronous queries.

```python
cluster = Cluster(
    contact_points=[
        "your-node-url-1.clusters.scylla.cloud",
        "your-node-url-2.clusters.scylla.cloud", 
        "your-node-url-3.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='your-awesome-password')
)

session = cluster.connect()

results = session.execute('SELECT * FROM system.clients LIMIT 10')

for item in results:
    print(item)
    print(item.address)

```

Response will be a ResultSet of Rows: 

```
Row(address='123.123.123.123', port=46160, client_type='cql', connection_stage='AUTHENTICATING', driver_name='Scylla Python Driver', driver_version='3.26.2', hostname=None, protocol_version=4, shard_id=0, ssl_cipher_suite=None, ssl_enabled=None, ssl_protocol=None, username='scylla')
123.123.123.123
```


### 3.1 Creating a Keyspace

The `keyspace` inside the ScyllaDB ecosystem can be interpreted as your `database` or `collection`.

On your connection boot, you don't need to provide it but you will use it later and also is able to create when you need.

```python

cluster = Cluster(
    contact_points=[
        "your-node-url-1.clusters.scylla.cloud",
        "your-node-url-2.clusters.scylla.cloud", 
        "your-node-url-3.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='your-awesome-password')
)

session = cluster.connect()

keyspaceName = "media_player"
replicationFactor = 3
session.execute(
    f"""
    CREATE KEYSPACE {keyspaceName}
        WITH replication = {{'class': 'NetworkTopologyStrategy', 'replication_factor': '{replicationFactor}'}}
        AND durable_writes = true;
    """
)

session.set_keyspace('media_player')
```

Unfortunatelly you can't set a keyspace with `PreparedStatements`, so you will need to build the query by yourself.

> You can use the `session.set_keyspace()` function to switch between keyspaces or set it on your cluster connection.

### 3.2 Creating a Table

A table is used to store part or all of your app data (depending on how structure your database schema). 
Add the `keyspace` as a parameter in the connection object and define a CQL string that creates a table to store your favorite songs.

```python
cluster = Cluster(
    contact_points=[
        "your-node-url-1.clusters.scylla.cloud",
        "your-node-url-2.clusters.scylla.cloud", 
        "your-node-url-3.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='your-awesome-password')
)

session = cluster.connect('media_player')

tableQuery = """
CREATE TABLE songs (
    id uuid,
    title text,
    album text,
    artist text,
    created_at timestamp,
    PRIMARY KEY (id, created_at)
)
"""

session.execute(tableQuery)
```

### 3.3 Inserting data

Now that you have created a keyspace and a table, you need to insert some songs to populate the table. 

```python

cluster = Cluster(
    contact_points=[
        "node-0.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
        "node-1.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud", 
        "node-2.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud"
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='r4GnOL2QSDi1wqF')
)

session = cluster.connect('media_player')

songList = [
    {
        "id": 'd754f8d5-e037-4898-af75-44587b9cc424',
        "title": 'Stairway to Heaven',
        "album": 'Led Zeppelin III',
        "artist": 'Led Zeppelin',
        "createdAt": datetime.datetime.now(),
    },
    {
        "id": uuid4(),
        "title": 'Glimpse of Us',
        "album": 'Smithereens',
        "artist": 'Joji',
        "createdAt": datetime.datetime.now(),
    },
    {
        "id": uuid4(),
        "title": 'Vegas',
        "album": 'From Movie ELVIS',
        "artist": 'Doja Cat',
        "createdAt": datetime.datetime.now(),
    },
];

query = session.prepare("""
    INSERT INTO songs (id, title, album, artist, created_at) VALUES (?, ?, ?, ?, ?)
""")

for music in songList:
    session.execute(query, music.values())

```

### 3.3 Reading data

Since probably we added more than 3 songs into our database, let's list it into our terminal.

```python
cluster = Cluster(
    contact_points=[
        "node-0.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='r4GnOL2QSDi1wqF')
)

session = cluster.connect('media_player')

query = session.prepare("SELECT * FROM songs");

results = session.execute(query);

for song in results: 
    print(song.id)
    print(song.title)
```

The result looks like this:

```
0a5d5cfb-0275-43a4-9523-45ad256666f8
Vegas
```

### 3.4 Updating Data

Ok, almost there! Now we're going to learn about `UPDATE` but here's a disclaimer: 
> INSERT and UPDATES are not the same!

There's a myth in Scylla/Cassandra community that it's the same for the fact that you just need the `Partition Key` and `Clustering Key` (if you have one) and query it.

Read more about [`INSERT` and `UPDATE`](https://docs.scylladb.com/stable/using-scylla/cdc/cdc-basic-operations.html)

As you can see, the `UPDATE` query takes two fields in the `WHERE` clause (PK and CK). Check the snippet below: 

```js
const songToUpdate = {
    id: 'd754f8d5-e037-4898-af75-44587b9cc424',
    title: 'Glimpse of Us',
    album: 'Smithereens',
    artist: 'Joji',
};

const updateSong = async (songToUpdate) => {
    const cluster = new cassandra.Client({
        contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
        localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
        credentials: {username: 'scylla', password: '********'},
        keyspace: 'media_player'
    })

    let results = await cluster.execute("SELECT * FROM songs");
    let rows = results.rows;

    let songToUpdate = rows.find((row) => row.id.toString() === song.id)
    
    let query = await cluster.execute(`UPDATE songs set title = 'Glimpse of US - Inutilismo' where id = ${songToUpdate.id} AND updated_at = '2023-03-02 23:10:00.00+0000';`);

    await cluster.shutdown()
}
```
After the data gets inserted, query all columns and filter by the ID:
```
scylla@cqlsh:media_player> select * from songs where id = d754f8d5-e037-4898-af75-44587b9cc424;

 id                                   | updated_at                      | album       | artist | created_at                      | title
--------------------------------------+---------------------------------+-------------+--------+---------------------------------+----------------------------
 d754f8d5-e037-4898-af75-44587b9cc424 | 2023-03-02 22:00:00.000000+0000 | Smithereens |   Joji | 2023-03-02 22:00:00.000000+0000 |              Glimpse of Us
 d754f8d5-e037-4898-af75-44587b9cc424 | 2023-03-02 23:10:00.000000+0000 |        null |   null |                            null | Glimpse of US - Inutilismo
```

It only updated the field `title` and `updated_at` (the Clustering Key) and since we didn't input the rest of the data, it will not be replicated as expected.


### 3.5 Deleting Data

Last things last! Let's understand what we can DELETE with this statement. There's the normal `DELETE` statement that focus on `ROWS` and other one that delete data only from `COLUMNS` and the syntax is very similar.

```sql 
// Deletes a single row
DELETE FROM songs WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;

// Deletes a whole column
DELETE artist FROM songs WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;
```

If you want to erase a specific column, you also should pass as parameter the `Clustering Key` and be very specific in which register you want to delete something. 
On the other hand, the "normal delete" just need the `Partition Key` to handle it. Just remember: if you use the statement "DELETE FROM <table>" it will delete ALL the rows that you stored with that ID. 

```js
const deleteColumnFromSong = async (song) => {
    const cluster = new cassandra.Client({
        contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
        localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
        credentials: {username: 'scylla', password: '********'},
        keyspace: 'media_player'
    })

    await cluster.execute(`DELETE artist FROM songs WHERE id = ${song.id} AND updated_at = '${song.updatedAt}'`)
    await cluster.shutdown()
}

const deleteSong = async (song) => {
    const cluster = new cassandra.Client({
        contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
        localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
        credentials: {username: 'scylla', password: '********'},
        keyspace: 'media_player'
    })

    await cluster.execute(`DELETE FROM songs WHERE id = ${song.id}`)
    await cluster.shutdown()
}
```

## Conclusion

Yay! You now have the knowledge to use the basics of ScyllaDB with NodeJS.

If you thinks that something can be improved, please open an issue and let's make it happen!


Did you like the content? [Tweet about it](https://twitter.com/intent/tweet?url=https%3A%2F%2Fgithub.com%2Fscylladb%2Fscylla-cloud-getting-started&via=scylladb%20%40danielhe4rtless&text=Just%20finished%20the%20ScyllaDB%20Hello%20World%20in%20NodeJs&hashtags=scylladb%20%23nodejs)!


