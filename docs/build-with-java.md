# Quick Start: Java with ScyllaDB

In this tutorial, you'll build a Media Player to store your songs and build playlists.

## 1. Getting the Driver

Install the [Java ScyllaDB Driver](https://java-driver.docs.scylladb.com/scylla-3.11.2.x/index.html).
```
<dependency>
  <groupId>com.scylladb</groupId>
  <artifactId>java-driver-core</artifactId>
  <version>3.11.2.0</version>
</dependency>

<dependency>
  <groupId>com.scylladb</groupId>
  <artifactId>java-driver-query-builder</artifactId>
  <version>3.11.2.0</version>
</dependency>

<dependency>
  <groupId>com.scylladb</groupId>
  <artifactId>java-driver-mapper-runtime</artifactId>
  <version>3.11.2.0</version>
</dependency>
```

## 2. Connecting to the Cluster

Get your database credentials from your [ScyllaDB Cloud Cluster View](https://cloud.scylladb.com/clusters) in the tab `Connect`.

```java
import com.datastax.driver.core.Cluster;  
import com.datastax.driver.core.PlainTextAuthProvider;  
import com.datastax.driver.core.Session;  
import com.datastax.driver.core.policies.DCAwareRoundRobinPolicy;  
  
class Main {  
  
    public static void main(String[] args) {  
    Cluster cluster = Cluster.builder()  
        .addContactPoints("your-node-url.scylla.cloud", "your-node-url.clusters.scylla.cloud", "your-node-url.clusters.scylla.cloud")  
        .withLoadBalancingPolicy(DCAwareRoundRobinPolicy.builder().withLocalDc("AWS_US_EAST_1").build()) // your local data center  
        .withAuthProvider(new PlainTextAuthProvider("scylla", "7H8VbxJTuG2flYK"))  
        .build();  
    
    Session session = cluster.connect();  
    
    }  
}
```

> If the connection gets refused, check if your IP Address is added to the list of allowed IP addresses.

## 3. Handling Queries

With the Java driver, you can use the function inside your cluster connection called `execute(query)` and build the query you want to execute inside your database/keyspace.

```java
import com.datastax.driver.core.Cluster;
import com.datastax.driver.core.ResultSet;
import com.datastax.driver.core.Session;
import com.datastax.driver.core.PlainTextAuthProvider;
import com.datastax.driver.core.policies.DCAwareRoundRobinPolicy;

class Main {  
  
    public static void main(String[] args) {  
        Cluster cluster = Cluster.builder()  
            .addContactPoints("your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud")  
            .withLoadBalancingPolicy(DCAwareRoundRobinPolicy.builder().withLocalDc("AWS_US_EAST_1").build()) // your local data center  
            .withAuthProvider(new PlainTextAuthProvider("scylla", "*******"))  
            .build();  
        
        Session session = cluster.connect();  
        ResultSet result = session.execute("SELECT * FROM system.clients LIMIT 10");  
  
        System.out.println(result);
    }
}
```


### 3.1 Creating a Keyspace

A Keyspace in ScyllaDB is a collection of tables with attributes which define how data is replicated on nodes. 

You don't need a keyspace on your connection boot, but you'll need it to create a table.

```java
import com.datastax.driver.core.Cluster;
import com.datastax.driver.core.ResultSet;
import com.datastax.driver.core.Session;
import com.datastax.driver.core.PlainTextAuthProvider;
import com.datastax.driver.core.policies.DCAwareRoundRobinPolicy;

import static com.datastax.driver.core.schemabuilder.SchemaBuilder.createKeyspace;

class Main {  
  
    public static void main(String[] args) { 
        Cluster cluster = Cluster.builder()  
            .addContactPoints("your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud")  
            .withLoadBalancingPolicy(DCAwareRoundRobinPolicy.builder().withLocalDc("AWS_US_EAST_1").build()) // your local data center  
            .withAuthProvider(new PlainTextAuthProvider("scylla", "*******"))  
            .build();  
            
            Session session = cluster.connect();  

            String createKeyspaceQuery = createKeyspace("media_player")  
                .ifNotExists()  
                .with()  
                .replication(ImmutableMap.of("class", "NetworkTopologyStrategy", "replication_factor", 3))  
                .durableWrites(true)  
                .getQueryString();

            session.execute(createKeyspaceQuery)
     }
}
```

After that you probably will need to re-create your connection poiting which `keyspace` you want to use.

### 3.2 Creating a Table

A table stores part or all of your app data (depending on how you structure your database schema). 
Add the `keyspace` as a parameter in the connection object and define a CQL string that creates a table to store your favorite songs.

```java
import com.datastax.driver.core.Cluster;
import com.datastax.driver.core.ResultSet;
import com.datastax.driver.core.Session;
import com.datastax.driver.core.PlainTextAuthProvider;
import com.datastax.driver.core.policies.DCAwareRoundRobinPolicy;

import static com.datastax.driver.core.schemabuilder.SchemaBuilder.createTable;

class Main {  
  
    public static void main(String[] args) { 
        Cluster cluster = Cluster.builder()  
            .addContactPoints("your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud")  
            .withLoadBalancingPolicy(DCAwareRoundRobinPolicy.builder().withLocalDc("AWS_US_EAST_1").build()) // your local data center  
            .withAuthProvider(new PlainTextAuthProvider("scylla", "*******"))  
            .build();  
            
            Session session = cluster.connect();  

            String createTableQuery = createTable("media_player", "songs")
                .ifNotExists()
                .addPartitionKey("id", DataType.bigint())
                .addClusteringColumn("updated_at", DataType.timestamp())
                .addColumn("title", DataType.text())
                .addColumn("album", DataType.text())
                .addColumn("artist", DataType.text())
                .addColumn("created_at", DataType.timestamp())
                .getQueryString()

            session.execute(createTableQuery)
     }
}
```

### 3.3 Inserting data

Now that you have created a keyspace and a table, you need to insert some songs to populate the table. 

```java
import com.datastax.driver.core.Cluster;
import com.datastax.driver.core.BoundStatement;
import com.datastax.driver.core.PreparedStatement;
import com.datastax.driver.core.Session;
import com.datastax.driver.core.PlainTextAuthProvider;
import com.datastax.driver.core.policies.DCAwareRoundRobinPolicy;

import java.util.Date;
import java.util.Scanner;

class Main {  
  
    public static void main(String[] args) { 
        Cluster cluster = Cluster.builder()  
            .addContactPoints("your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud")  
            .withLoadBalancingPolicy(DCAwareRoundRobinPolicy.builder().withLocalDc("AWS_US_EAST_1").build()) // your local data center  
            .withAuthProvider(new PlainTextAuthProvider("scylla", "*******"))  
            .build();  
            
            Session session = cluster.connect();  

            PreparedStatement statement = session.prepare(
                "INSERT INTO media_player.songs (id, title, artist, album, created_at, updated_at) VALUES (?,?,?,?,?,?)"
            );

            BoundStatement bound = statement.bind()
                    .setLong(0, 1)
                    .setString(1, "Stairway to Heaven")
                    .setString(2, "Led Zeppelin")
                    .setString(3, "Led Zeppelin IV")
                    .setTimestamp(4, new Date())
                    .setTimestamp(5, new Date());

            session.execute(bound);
     }
}
```

### 3.3 Reading data

Let's read the songs from the database and print them to the terminal.

```java
import com.datastax.driver.core.Cluster;
import com.datastax.driver.core.BoundStatement;
import com.datastax.driver.core.PreparedStatement;
import com.datastax.driver.core.Row;
import com.datastax.driver.core.Session;
import com.datastax.driver.core.PlainTextAuthProvider;
import com.datastax.driver.core.policies.DCAwareRoundRobinPolicy;

import java.util.Date;
import java.util.Scanner;

class Main {  
  
    public static void main(String[] args) { 
        Cluster cluster = Cluster.builder()  
            .addContactPoints("your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud")  
            .withLoadBalancingPolicy(DCAwareRoundRobinPolicy.builder().withLocalDc("AWS_US_EAST_1").build()) // your local data center  
            .withAuthProvider(new PlainTextAuthProvider("scylla", "*******"))  
            .build();  
            
            Session session = cluster.connect();  

            ResultSet songsResult = session.execute("SELECT * FROM media_player.songs PER PARTITION LIMIT 1");
            List<Row> songList = results.all();

            for (Row row : resultsmemo) {
                System.out.println(
                    String.format(
                            "ID: %d -> Song: %s -> Artist: %s -> Album: %s -> Created At: %s",
                            row.getInt("id"),
                            row.getString("title"),
                            row.getString("artist"),
                            row.getString("album"),
                            row.getTimestamp("created_at").toString()
                    )
            );
        }
     }
}
```


### 3.4 Updating Data

The `UPDATE` query in the fact is equals to `INSERT` regarding the syntax. Uou just need the `Partition Key` and `Clustering Key` (if you have one) and query it.

The `UPDATE` query takes two fields in the `WHERE` clause (PK and CK). See the snippet below: 


```java
import com.datastax.driver.core.Cluster;
import com.datastax.driver.core.BoundStatement;
import com.datastax.driver.core.PreparedStatement;
import com.datastax.driver.core.Session;
import com.datastax.driver.core.PlainTextAuthProvider;
import com.datastax.driver.core.policies.DCAwareRoundRobinPolicy;

import java.util.Date;
import java.util.Scanner;

class Main {  
  
    public static void main(String[] args) { 
        Cluster cluster = Cluster.builder()  
            .addContactPoints("your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud")  
            .withLoadBalancingPolicy(DCAwareRoundRobinPolicy.builder().withLocalDc("AWS_US_EAST_1").build()) // your local data center  
            .withAuthProvider(new PlainTextAuthProvider("scylla", "*******"))  
            .build();  
            
            Session session = cluster.connect();  

            PreparedStatement statement = session.prepare(
                "UPDATE songs set title = ? where id = ? AND updated_at = ?"
            );

            BoundStatement bound = statement.bind()
                    .setLong(0, 1)
                    .setString(1, "Rock and Roll")                    
                    .setTimestamp(2, new Date());

            session.execute(bound);
     }
}
```

After the data gets inserted, query all columns and filter by the ID:
```
scylla@cqlsh:media_player> select * from songs where id = 1;

id | updated_at                      | album       | artist | created_at                      | title
---+---------------------------------+-------------+--------+---------------------------------+----------------------------
 1 | 2023-03-02 22:00:00.000000+0000 | Smithereens |   Joji | 2023-03-02 22:00:00.000000+0000 |              Glimpse of Us
 1 | 2023-03-02 23:10:00.000000+0000 |        null |   null |                            null | Glimpse of US - Inutilismo
```

It only updated the field `title` and `updated_at` (the Clustering Key), and since we didn't input the rest of the data, it will not be replicated as expected.


### 3.5 Deleting Data

Last things last! Let's understand what we can DELETE with this statement. There's the regular `DELETE` statement that focuses on `ROWS` and the other one that deletes data only from `COLUMNS`. The syntax is very similar.

```sql 
// Deletes a single row
DELETE FROM songs WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;

// Deletes a cell
DELETE artist FROM songs WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;
```

If you want to erase a specific column, you also should pass as parameter the `Clustering Key` and be very specific in which register you want to delete something. 
On the other hand, the "normal delete" just need the `Partition Key` to handle it. Just remember: if you use the statement "DELETE FROM <table>" it will delete ALL the rows that you stored with that ID. 

```java
import com.datastax.driver.core.Cluster;
import com.datastax.driver.core.BoundStatement;
import com.datastax.driver.core.PreparedStatement;
import com.datastax.driver.core.Session;
import com.datastax.driver.core.PlainTextAuthProvider;
import com.datastax.driver.core.policies.DCAwareRoundRobinPolicy;

import java.util.Date;
import java.util.Scanner;

class Main {  
  
    public static void main(String[] args) { 
        Cluster cluster = Cluster.builder()  
            .addContactPoints("your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud", "your-node-url.7207ca5a8fdc45f2b03f.clusters.scylla.cloud")  
            .withLoadBalancingPolicy(DCAwareRoundRobinPolicy.builder().withLocalDc("AWS_US_EAST_1").build()) // your local data center  
            .withAuthProvider(new PlainTextAuthProvider("scylla", "*******"))  
            .build();  
            
            Session session = cluster.connect();  

            PreparedStatement statement = session.prepare(
                "DELETE FROM songs where id = ?"
            );

            BoundStatement bound = statement.bind()
                    .setLong(0, 1);

            session.execute(bound);
     }
}
```

## Conclusion

Yay! You now have the knowledge to use the basics of ScyllaDB with Java.

There is a simple project with this structure that you can check out [here](https://github.com/DanielHe4rt/scylladb-java-getting-started).

If you think that something can be improved, please open an issue, and let's make it happen!


Did you like the content? [Tweet about it](https://twitter.com/intent/tweet?url=https%3A%2F%2Fgithub.com%2Fscylladb%2Fscylla-cloud-getting-started&via=scylladb%20%40danielhe4rtless&text=Just%20finished%20the%20ScyllaDB%20Hello%20World%20in%20Java&hashtags=scylladb%20%23java)!