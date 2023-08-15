# Quick start: JavaScript (Node.js)

In this tutorial you'll build a Media Player to store your songs and build playlists.

## 1. Getting the Driver

Install the [JavaScript Cassandra driver](https://github.com/datastax/nodejs-driver/) that also works with ScyllaDB.

```sh
$ npm install cassandra-driver

$ yarn install cassandra-driver
```

## 2. Connect to the cluster

Get your database credentials from your [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) in the tab `Connect`.

> Add your machine's IP Address to the list of allowed IP addresses in ScyllaDB Cloud. Otherwise, your connection will get refused.

```js
const cluster = new cassandra.Client({
    contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
    localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
    credentials: {username: 'scylla', password: 'your-awesome-password'},
    // keyspace: 'your_keyspace' // optional
})
```

## 3. Handling Queries

With the NodeJS driver, you can use the function inside your cluster connection called `execute(query)` and build the query you want to execute inside your database/keyspace.

```js
const cluster = new cassandra.Client({
    contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
    localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
    credentials: {username: 'scylla', password: 'your-awesome-password'}
})

const results = await cluster.execute('SELECT * FROM system.clients LIMIT 10')
console.log(results);
results.rows.forEach(row => console.log(JSON.stringify(row)))
```

### 3.1 Create a keyspace

The `keyspace` inside the ScyllaDB ecosystem can be interpreted as your `database` or `collection`.

On your connection boot, you don't need to provide it but you will use it later and also is able to create when you need.

```js
async function runKeyspace () {
    const cluster = new cassandra.Client({
        contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
        localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
        credentials: {username: 'scylla', password: 'your-awesome-password'},
    })

    const newKeyspace = (keyspaceName, rf) => `
        CREATE KEYSPACE ${keyspaceName}
            WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '${rf}'} 
            AND durable_writes = true;
    `;

    await cluster.execute(newKeyspace('media_player', 3))
    await cluster.shutdown()
}


runKeyspace();
```

> After that you probably will need to re-create your connection poiting which `keyspace` you want to use.

### 3.2 Creating a Table

A table is used to store part or all of your app data (depending on how structure your database schema). 
Add the `keyspace` as a parameter in the connection object and define a CQL string that creates a table to store your favorite songs.

```js
async function runKeyspace (keyspace = null) {
    const cluster = new cassandra.Client({
        contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
        localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
        credentials: {username: 'scylla', password: 'your-awesome-password'},
        keyspace: keyspace ?? 'media_player'
    })

    const createSongsTableQuery = `
        CREATE TABLE songs (
            id int,
            title text,
            album text,
            artist text,
            created_at timestamp,
            updated_at timestamp
            PRIMARY KEY (id, updated_at)
        )
    `;

    await cluster.execute(createSongsTableQuery))
    await cluster.shutdown()
}


runKeyspace('media_player');
```

### 3.3 Inserting data

Now that you have created a keyspace and a table, you need to insert some songs to populate the table. 

```js
async function insertSongs () {
    const cluster = new cassandra.Client({
        contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
        localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
        credentials: {username: 'scylla', password: 'your-awesome-password'},
        keyspace: keyspace ?? 'media_player'
    })

    let songList = [
        {
            id: cassandra.types.Uuid.random(),
            title: 'Stairway to Heaven',
            album: 'Led Zeppelin IV',
            artist: 'Led Zeppelin',
            createdAt: '2023-03-02 22:00:00',
            updatedAt: '2023-03-02 22:00:00',
        }, 
        {
            id: 'd754f8d5-e037-4898-af75-44587b9cc424',
            title: 'Glimpse of Us',
            album: 'Smithereens',
            artist: 'Joji',
            createdAt: '2023-03-02 22:00:00',
            updatedAt: '2023-03-02 22:00:00',
        },
        {
            id: cassandra.types.Uuid.random(),
            title: 'Vegas',
            album: 'From Movie ELVIS',
            artist: 'Doja Cat',
            createdAt: '2023-03-02 22:00:00',
            updatedAt: '2023-03-02 22:00:00',
        },
    ];

    const newSongQuery = (song) => {
        return `INSERT INTO songs (id, title, album, artist) 
                    VALUES (${song.id}, '${song.title}', '${song.album}', '${song.artist}', '${song.createdAt}', '${song.updatedAt}')`

    }

    for (let i in songList) {
        await cluster.execute(newSongQuery(songList[i]))
    }
    
    
    await cluster.shutdown()
}
```

### 3.3 Reading data

Since probably we added more than 3 songs into our database, let's list it into our terminal.

```js
const listSongs = async () => {
    
    const cluster = new cassandra.Client({
        contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
        localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
        credentials: {username: 'scylla', password: 'your-awesome-password'},
        keyspace: keyspace ?? 'media_player'
    })

    let results = await cluster.execute("SELECT * FROM songs");
    let rows = results.rows;

    for (let i in rows) {
        console.log(rows[i])
        console.log(rows[i].id.toString())
    }

    await cluster.shutdown()
}
```

The result looks like this:

```
$ node index.js 

Row {
  id: Uuid {
    buffer: <Buffer d7 54 f8 d5 e0 37 48 98 af 75 44 58 7b 9c c4 24>
  },
  title: 'Glimpse of Us',
  album: 'Smithereens',
  artist: 'Joji'
}
...
```

> Remeber to decode your Uuid if needed using the function `.toString()`

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
        credentials: {username: 'scylla', password: 'your-awesome-password'},
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
On the other hand, the "normal delete" just need the `Partition Key` to handle it. Just remember: if you use the statement `DELETE FROM <table>` it will delete ALL the rows that you stored with that ID. 

```js
const deleteColumnFromSong = async (song) => {
    const cluster = new cassandra.Client({
        contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
        localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
        credentials: {username: 'scylla', password: 'your-awesome-password'},
        keyspace: 'media_player'
    })

    await cluster.execute(`DELETE artist FROM songs WHERE id = ${song.id} AND updated_at = '${song.updatedAt}'`)
    await cluster.shutdown()
}

const deleteSong = async (song) => {
    const cluster = new cassandra.Client({
        contactPoints: ["your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud", ...],
        localDataCenter: 'your-data-center', // Eg: AWS_SA_EAST_1
        credentials: {username: 'scylla', password: 'your-awesome-password'},
        keyspace: 'media_player'
    })

    await cluster.execute(`DELETE FROM songs WHERE id = ${song.id}`)
    await cluster.shutdown()
}
```

## Conclusion

Yay! You now know how get started with ScyllaDB in Node.js.

If you think something can be improved, please open an issue and let's make it happen!

Did you like the content? Don't forget to star the repo and follow us on socials.