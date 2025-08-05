# Design and Data Model

You can learn more about Data Modeling in Scylla (and NoSQL) by taking
[this course](https://university.scylladb.com/courses/data-modeling/) on Scylla
University. The main goal of data modeling in Scylla is to perform queries fast,
even if we sometimes have to duplicate data.

Let's build our schema around the queries we are going to run against our domain
entities. When creating the data model, you need to consider both the conceptual
data model and the application workflow: which queries will be performed by
which users and how often.

To achieve that, we want:

-   Even data distribution
-   To minimize the number of partitions accessed in a read query.

On the other hand, our focus won't be on avoiding data duplication or minimizing
the number of writes. You're probably familiar with the steps defined here:

![](https://lh5.googleusercontent.com/5JqE89v8KJbSuVsnGswHn83sJOV-tjpeH6r1fqdNl6S77ncqAYb3kIZPSgNI8bqN_43OyZNbHQVpXdqMBFrRmsEvG3JORR302EhMnIb9qa6nuNL7cP2JJDZ4Uon_Pp-QmSCoEQ)

## Conceptual Data Model

Starting with the conceptual data model, we need to identify the key entities
and the relationships between them. Our application is focused in a single entity called 'songs'. The concept is to fill with many songs as we want. 


![Table Diagram](/_static/img/getting-started-diagram.png)

## Application Workflow

Next, we move on to the Application Workflow. In this part, we identify the main
queries or what questions we will ask the database. This part is important in
Scylla, and other NoSQL databases and, as opposed to relational databases is
performed early on in the data modeling process. Remember that our data modeling
is built around the queries.

## Application Features

* Insert a Song
* List all songs
* Delete a song
* Stress Testing

## Queries

Now we can detail the above queries in
[CQL](https://university.scylladb.com/courses/data-modeling/lessons/basic-data-modeling-2/topic/cql-cqlsh-and-basic-cql-syntax/):

Q1: Create a Keyspace

    CREATE KEYSPACE prod_media_player
            WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'} 
            AND durable_writes = true;

Q2: Create a Table

    CREATE TABLE songs (
            id int,
            title text,
            album text,
            artist text,
            created_at timestamp,
            updated_at timestamp
            PRIMARY KEY (id, updated_at)
    );

Q3: Insert a new song

    INSERT INTO prod_media_player.songs (id,title,artist,album,created_at) VALUES (?,?,?,?,?);

Q4: List all songs

    SELECT * FROM songs;

Q5: Delete a specific song

    DELETE FROM songs WHERE id = ?


## Helpful Material


Some more advanced topics not covered in this guide are
[Collections](https://university.scylladb.com/courses/data-modeling/lessons/advanced-data-modeling/topic/common-data-types-and-collections/),
User-Defined[Types](https://university.scylladb.com/courses/data-modeling/lessons/advanced-data-modeling/topic/user-defined-types-udt/)
(UDT), expiring data with [time to live (TTL)](https://university.scylladb.com/courses/data-modeling/lessons/advanced-data-modeling/topic/expiring-data-with-ttl-time-to-live/),
and [Counters](https://university.scylladb.com/courses/data-modeling/lessons/advanced-data-modeling/topic/counters/).

To summarize, when data modeling with Scylla, we have to know our data, think
about our queries, pay attention to the primary key and clustering key
selection, and not be afraid to duplicate data.

