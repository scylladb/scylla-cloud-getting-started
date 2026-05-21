# Quick start: Java

In this tutorial you'll build a Media Player to store your songs and build playlists.

## 1. Getting the Driver

Add the [ScyllaDB Java Driver](https://java-driver.docs.scylladb.com/) to your `pom.xml`:

```xml
<dependency>
  <groupId>com.scylladb</groupId>
  <artifactId>java-driver-core</artifactId>
  <version>4.18.0.0</version>
</dependency>
```

To package your application as a fat JAR (all dependencies included), use the `maven-shade-plugin`:

```xml
<plugin>
  <groupId>org.apache.maven.plugins</groupId>
  <artifactId>maven-shade-plugin</artifactId>
  <version>3.5.1</version>
  <executions>
    <execution>
      <phase>package</phase>
      <goals><goal>shade</goal></goals>
      <configuration>
        <outputFile>${project.build.directory}/app.jar</outputFile>
        <transformers>
          <transformer implementation="org.apache.maven.plugins.shade.resource.ManifestResourceTransformer">
            <mainClass>com.scylladb.App</mainClass>
          </transformer>
          <transformer implementation="org.apache.maven.plugins.shade.resource.ServicesResourceTransformer"/>
          <transformer implementation="org.apache.maven.plugins.shade.resource.AppendingTransformer">
            <resource>reference.conf</resource>
          </transformer>
        </transformers>
      </configuration>
    </execution>
  </executions>
</plugin>
```

## 2. Connecting to the cluster

Get your database credentials from your [ScyllaDB Cloud Cluster View](https://cloud.scylladb.com/clusters) in the tab `Connect`.

> Add your machine's IP Address to the list of allowed IP addresses in ScyllaDB Cloud. Otherwise, your connection will get refused.

```java
import com.datastax.oss.driver.api.core.CqlSession;
import java.net.InetSocketAddress;

public class App {
    public static void main(String[] args) {
        CqlSession session = CqlSession.builder()
            .addContactPoint(new InetSocketAddress("node-0.aws-us-east-1.xxxx.clusters.scylla.cloud", 9042))
            .withAuthCredentials("scylla", "your-awesome-password")
            .withLocalDatacenter("AWS_US_EAST_1")
            .build();
    }
}
```

## 3. Handling Queries

With the ScyllaDB Java driver 4.x, use `session.execute(query)` to run CQL statements.

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.cql.ResultSet;
import com.datastax.oss.driver.api.core.cql.Row;
import java.net.InetSocketAddress;

public class App {
    public static void main(String[] args) {
        CqlSession session = CqlSession.builder()
            .addContactPoint(new InetSocketAddress("node-0.aws-us-east-1.xxxx.clusters.scylla.cloud", 9042))
            .withAuthCredentials("scylla", "your-awesome-password")
            .withLocalDatacenter("AWS_US_EAST_1")
            .build();

        ResultSet result = session.execute("SELECT * FROM system.clients LIMIT 10");
        for (Row row : result) {
            System.out.println(row.toString());
        }

        session.close();
    }
}
```


### 3.1 Create a keyspace

A keyspace in ScyllaDB is a collection of tables with attributes which define how data is replicated on nodes.

```java
import com.datastax.oss.driver.api.core.CqlSession;
import java.net.InetSocketAddress;

public class App {
    public static void main(String[] args) {
        CqlSession session = CqlSession.builder()
            .addContactPoint(new InetSocketAddress("node-0.aws-us-east-1.xxxx.clusters.scylla.cloud", 9042))
            .withAuthCredentials("scylla", "your-awesome-password")
            .withLocalDatacenter("AWS_US_EAST_1")
            .build();

        session.execute(
            "CREATE KEYSPACE IF NOT EXISTS media_player " +
            "WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'} " +
            "AND durable_writes = true"
        );

        session.close();
    }
}
```

### 3.2 Create table

A table stores part or all of your app data (depending on how you structure your database schema).
Add the `keyspace` prefix in your table name and define a CQL string that creates a table to store your favorite songs.

```java
import com.datastax.oss.driver.api.core.CqlSession;
import java.net.InetSocketAddress;

public class App {
    public static void main(String[] args) {
        CqlSession session = CqlSession.builder()
            .addContactPoint(new InetSocketAddress("node-0.aws-us-east-1.xxxx.clusters.scylla.cloud", 9042))
            .withAuthCredentials("scylla", "your-awesome-password")
            .withLocalDatacenter("AWS_US_EAST_1")
            .build();

        session.execute(
            "CREATE TABLE IF NOT EXISTS media_player.playlist (" +
            "  id uuid," +
            "  title text," +
            "  album text," +
            "  artist text," +
            "  created_at timestamp," +
            "  PRIMARY KEY (id, created_at)" +
            ") WITH CLUSTERING ORDER BY (created_at DESC)"
        );

        session.close();
    }
}
```

### 3.3 Insert data

Now that you have created a keyspace and a table, insert some songs to populate the table.

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.cql.PreparedStatement;
import java.net.InetSocketAddress;
import java.time.Instant;
import java.util.UUID;

public class App {
    public static void main(String[] args) {
        CqlSession session = CqlSession.builder()
            .addContactPoint(new InetSocketAddress("node-0.aws-us-east-1.xxxx.clusters.scylla.cloud", 9042))
            .withAuthCredentials("scylla", "your-awesome-password")
            .withLocalDatacenter("AWS_US_EAST_1")
            .build();

        PreparedStatement ps = session.prepare(
            "INSERT INTO media_player.playlist (id, title, artist, album, created_at) VALUES (?, ?, ?, ?, ?)"
        );

        session.execute(ps.bind(
            UUID.randomUUID(),
            "Stairway to Heaven",
            "Led Zeppelin",
            "Led Zeppelin IV",
            Instant.now()
        ));

        session.close();
    }
}
```

### 3.4 Read data

Let's read the songs from the database and print them to the terminal.

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.cql.ResultSet;
import com.datastax.oss.driver.api.core.cql.Row;
import java.net.InetSocketAddress;

public class App {
    public static void main(String[] args) {
        CqlSession session = CqlSession.builder()
            .addContactPoint(new InetSocketAddress("node-0.aws-us-east-1.xxxx.clusters.scylla.cloud", 9042))
            .withAuthCredentials("scylla", "your-awesome-password")
            .withLocalDatacenter("AWS_US_EAST_1")
            .build();

        ResultSet rs = session.execute(
            "SELECT id, title, album, artist, created_at FROM media_player.playlist PER PARTITION LIMIT 1 LIMIT 100"
        );

        for (Row row : rs) {
            System.out.printf(
                "ID: %s -> Song: %s -> Artist: %s -> Album: %s -> Created At: %s%n",
                row.getUuid("id"),
                row.getString("title"),
                row.getString("artist"),
                row.getString("album"),
                row.getInstant("created_at")
            );
        }

        session.close();
    }
}
```


### 3.5 Update data

The `UPDATE` query takes two fields in the `WHERE` clause (`partition key` and `clustering key`). See the snippet below:

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.cql.PreparedStatement;
import java.net.InetSocketAddress;
import java.time.Instant;
import java.util.UUID;

public class App {
    public static void main(String[] args) {
        CqlSession session = CqlSession.builder()
            .addContactPoint(new InetSocketAddress("node-0.aws-us-east-1.xxxx.clusters.scylla.cloud", 9042))
            .withAuthCredentials("scylla", "your-awesome-password")
            .withLocalDatacenter("AWS_US_EAST_1")
            .build();

        UUID songId = UUID.fromString("d754f8d5-e037-4898-af75-44587b9cc424");

        PreparedStatement ps = session.prepare(
            "UPDATE media_player.song_counter SET times_played = times_played + 1 WHERE song_id = ?"
        );

        session.execute(ps.bind(songId));

        session.close();
    }
}
```

After the data gets inserted, query all columns and filter by the ID:
```
scylla@cqlsh:media_player> select * from playlist where id = d754f8d5-e037-4898-af75-44587b9cc424;

 id                                   | created_at                      | album           | artist      | title
--------------------------------------+---------------------------------+-----------------+-------------+------------------
 d754f8d5-e037-4898-af75-44587b9cc424 | 2023-03-02 22:00:00.000000+0000 | Led Zeppelin IV | Led Zeppelin | Stairway to Heaven
```

### 3.6 Delete Data

Last things last! Let's understand what we can DELETE with this statement. There's the regular `DELETE` statement that focuses on `ROWS` and the other one that deletes data only from `COLUMNS`. The syntax is very similar.

```sql
-- Deletes all rows for a partition
DELETE FROM media_player.playlist WHERE id = d754f8d5-e037-4898-af75-44587b9cc424;

-- Deletes a specific cell
DELETE artist FROM media_player.playlist WHERE id = d754f8d5-e037-4898-af75-44587b9cc424 AND created_at = '2023-03-02 22:00:00+0000';
```

If you want to remove a specific column, you also should pass the `Clustering Key` as parameter and be specific about which row you want to delete something from.
On the other hand, deleting by partition key only will delete ALL the rows stored with that ID.

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.cql.PreparedStatement;
import java.net.InetSocketAddress;
import java.util.UUID;

public class App {
    public static void main(String[] args) {
        CqlSession session = CqlSession.builder()
            .addContactPoint(new InetSocketAddress("node-0.aws-us-east-1.xxxx.clusters.scylla.cloud", 9042))
            .withAuthCredentials("scylla", "your-awesome-password")
            .withLocalDatacenter("AWS_US_EAST_1")
            .build();

        UUID songId = UUID.fromString("d754f8d5-e037-4898-af75-44587b9cc424");

        PreparedStatement ps = session.prepare(
            "DELETE FROM media_player.playlist WHERE id = ?"
        );

        session.execute(ps.bind(songId));

        session.close();
    }
}
```

## Conclusion

Yay! You now know how to get started with ScyllaDB in Java.

There is a simple project with this structure that you can check out [here](https://github.com/scylladb/scylla-cloud-getting-started/tree/main/java).

If you think something can be improved, please open an issue and let's make it happen!

Did you like the content? Don't forget to star the repo and follow us on socials.
