# Quick start: Python

In this tutorial you'll build a Media Player app to store your songs and create playlists.

## 1. Get the development environment ready

### 1.1 Prerequisites:
* [Python 3.7+](https://www.python.org/downloads/)
* [Virtualenv](https://virtualenv.pypa.io/en/latest/installation.html)


### 2.2 Set up the environment

Create a new folder for the project:
```sh
mkdir scylladb-cloud-python
cd scylladb-cloud-python
```

Create a new virtual environment and activate it:
```bash
virtualenv env
source env/bin/activate
```

Install the [Python ScyllaDB Driver](https://pypi.org/project/scylla-driver/).

```bash
pip install scylla-driver
```

## 2. Connect to the Cluster

Get your database credentials from your [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) in the tab `Connect`.

> Add your machine's IP Address to the list of allowed IP addresses in ScyllaDB Cloud. Otherwise, your connection will get refused.

```python
from cassandra.cluster import Cluster 
from cassandra.auth import PlainTextAuthProvider
cluster = Cluster(
    contact_points=[
        "your-node-url-1.clusters.scylla.cloud",
        "your-node-url-2.clusters.scylla.cloud", 
        "your-node-url-3.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='your-awesome-password')
)
```

## 3. Handling Queries

With the Python driver, you can use the function inside your cluster connection called `execute(query)` and build the query you want to execute inside your database/keyspace. You also can use the `execute_async()` for asynchronous queries.

```python
from cassandra.cluster import Cluster
from cassandra.auth import PlainTextAuthProvider
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

### 3.1 Create a Keyspace

The `keyspace` inside the ScyllaDB ecosystem can be interpreted as your `database` or `collection`.

On your connection boot, you don't need to provide it but you will use it later and also is able to create when you need.

{% raw %}
```python
from cassandra.cluster import Cluster
from datetime import datetime 
from cassandra.auth import PlainTextAuthProvider

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
    """
    CREATE KEYSPACE {}
        WITH replication = {{'class': 'NetworkTopologyStrategy', 'replication_factor': '{}'}}
        AND durable_writes = true;
    """.format(keyspaceName, replicationFactor)
)



session.set_keyspace('media_player')
```
{% endraw %}
Unfortunately you can't set a keyspace with `PreparedStatements`, so you will need to build the query by yourself.

> You can use the `session.set_keyspace()` function to switch between keyspaces.

### 3.2 Creating a Table

A table is used to store a part or all of your app data (depending on how you structure your database schema). 
Add the `keyspace` as a parameter in the connection object and define a CQL string that creates a table to store your favorite songs.

```python
from cassandra.cluster import Cluster
from datetime import datetime 
from cassandra.auth import PlainTextAuthProvider
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

### 3.3 Insert data

Now that you have created a keyspace and a table, insert some songs to populate the table. 

```python
from cassandra.cluster import Cluster
from datetime import datetime
from cassandra.auth import PlainTextAuthProvider
import uuid

cluster = Cluster(
    contact_points=[
        "your-node-url-1.clusters.scylla.cloud",
        "your-node-url-2.clusters.scylla.cloud", 
        "your-node-url-3.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='your-awesome-password')
)

session = cluster.connect('media_player')

songList = [
    {
        "id": uuid.UUID('d754f8d5-e037-4898-af75-44587b9cc424'),
        "title": 'Glimpse of Us',
        "album": 'Smithereens',
        "artist": 'Joji',
        "createdAt": datetime.now(),
    },
    {
        "id": uuid.uuid4(),
        "title": 'Stairway to Heaven',
        "album": 'Led Zeppelin III',
        "artist": 'Led Zeppelin',
        "createdAt": datetime.now(),
    },
    {
        "id": uuid.uuid4(),
        "title": 'Vegas',
        "album": 'From Movie ELVIS',
        "artist": 'Doja Cat',
        "createdAt": datetime.now(),
    },
];

query = session.prepare("""
    INSERT INTO songs (id, title, album, artist, created_at) VALUES (?, ?, ?, ?, ?)
""")

for music in songList:
    session.execute(query, music.values())

```

### 3.3 Read data

Since probably we added more than 3 songs into our database, let's list it into our terminal.

```python
from cassandra.cluster import Cluster
from datetime import datetime 
from cassandra.auth import PlainTextAuthProvider

cluster = Cluster(
    contact_points=[
        "your-node-url-1.clusters.scylla.cloud",
        "your-node-url-2.clusters.scylla.cloud", 
        "your-node-url-3.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='your-awesome-password')
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

### 3.4 Update data

The `UPDATE` query in the fact is equals to `INSERT` regarding the syntax. You just need the `Partition Key` and `Clustering Key` (if you have one) and query it.

The `UPDATE` query takes two fields in the `WHERE` clause (PK and CK). See the snippet below: 

```py
from cassandra.cluster import Cluster
from datetime import datetime 
from cassandra.auth import PlainTextAuthProvider
import uuid

cluster = Cluster(
    contact_points=[
        "your-node-url-1.clusters.scylla.cloud",
        "your-node-url-2.clusters.scylla.cloud", 
        "your-node-url-3.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='your-awesome-password')
)

session = cluster.connect('media_player')

songToUpdate = {
    "id": uuid.UUID('d754f8d5-e037-4898-af75-44587b9cc424'),
    "title": 'Glimpse of Us',
    "album": '2022 Em Uma Música',
    "artist": 'Lucas Inutilismo',
    "createdAt": datetime.now()
}

session.execute("""
    UPDATE songs SET 
        title = %(title)s, 
        album = %(album)s,
        artist = %(artist)s
    WHERE id = %(id)s AND created_at = %(createdAt)s
""", songToUpdate)
```

After the data gets inserted, query all columns and filter by the ID:

```
scylla@cqlsh:media_player> select * from songs where id = d754f8d5-e037-4898-af75-44587b9cc424;

 id                                   | created_at                      | album              | artist           | title
--------------------------------------+---------------------------------+--------------------+------------------+---------------
 d754f8d5-e037-4898-af75-44587b9cc424 | 2023-07-11 14:22:43.329000+0000 |        Smithereens |             Joji | Glimpse of Us
 d754f8d5-e037-4898-af75-44587b9cc424 | 2023-07-11 14:23:31.630000+0000 | 2022 Em Uma Música | Lucas Inutilismo | Glimpse of Us
 d754f8d5-e037-4898-af75-44587b9cc424 | 2023-07-11 14:23:51.996000+0000 | 2022 Em Uma Música |             null | Glimpse of Us

(3 rows)
```

In the snippet above, we updated the data fully but as you can see in the `SELECT` query, I ran one more update but without the `artist` column. So, as said before: the only "not nullable" fields are the `Partition Keys` and `Clustering Keys`.


### 3.5 Delete Data
Let's understand what we can DELETE with this statement. There's the normal `DELETE` statement that focuses on `ROWS` and another one that deletes data only from `COLUMNS` and the syntax is very similar.

```sql 
// Deletes a single row
DELETE FROM songs WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;

// Deletes a whole column
DELETE artist FROM songs WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;
```

If you want to remove a specific column, you also should pass as parameter the `Clustering Key` and be very specific in which register you want to delete something. 
On the other hand, the "normal delete" just need the `Partition Key` to handle it. Just remember: if you use the statement "DELETE FROM keyspace.table_name" it will delete ALL the rows that you stored with that ID. 

```py
from cassandra.cluster import Cluster
from datetime import datetime 
from cassandra.auth import PlainTextAuthProvider
import uuid

songToDelete = {
    "id": uuid.UUID('d754f8d5-e037-4898-af75-44587b9cc424'),
    "title": 'Glimpse of Us',
    "album": '2022 Em Uma Música',
    "artist": 'Lucas Inutilismo',
    "createdAt": datetime.now()
}

cluster = Cluster(
    contact_points=[
        "your-node-url-1.clusters.scylla.cloud",
        "your-node-url-2.clusters.scylla.cloud", 
        "your-node-url-3.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='your-awesome-password')
)

session = cluster.connect('media_player')


deleteQuery = session.prepare("DELETE FROM songs WHERE id = ? ")
session.execute(deleteQuery, [songToDelete['id']])

```

## 4. Conclusion

Yay! You now know how get started with ScyllaDB in Python.

If you think something can be improved, please open an issue and let's make it happen!

There is a sample project that you can learn more about the concepts and also have a good time testing our ScyllaDB Cloud Cluster, check it out [here](https://github.com/scylladb/scylla-cloud-getting-started/python).

Did you like the content? Don't forget to star the repo and follow us on socials.