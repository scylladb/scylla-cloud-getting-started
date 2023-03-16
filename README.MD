<p align="center">
    <img src="https://dev-to-uploads.s3.amazonaws.com/uploads/articles/u792bp8mle5zooqqlcy5.png" alt="">
</p>

# Getting started with Scylla Cloud

The project is designed to quickly get you through an experience to handle thousands of requests per second with a "Todo App" using [ScyllaDB](https://github.com/scylladb/scylladb) and [Scylla Cloud](https://www.scylladb.com/product/scylla-cloud/).

## Quick start

Language-specific implementation resides in the corresponding folder:

-  [PHP](/php/php82)
- Ruby (Not Implemented)
- Rust (Not implemented)

**Table of Contents:**

- [1. Before Start](#1-before-start)
- [2. Architecture](#2-architecture)
- [3. Scylla Cloud First Steps](#3-scylla-cloud-first-steps)
    - [3.1 Creating a cluster](#31-creating-a-cluster)
    - [3.2 Connect to your Cloud Cluster](#32-connect-to-your-cloud-cluster)
- [4. Building the App](#)
- [5. Contributing](#)

## 1. Before Start

We're going to use [ScyllaDB Cloud](https://www.scylladb.com/product/scylla-cloud/) you
can [register here](https://cloud.scylladb.com/user/signup) and try our database and platform.

Also, we're going to need [Docker](https://docs.docker.com/engine/install/) o connect into our ScyllaDB Cluster via
[CQLSH](https://docs.scylladb.com/stable/cql/cqlsh.html).

## 2. Architecture

Regarding of what we expect to have on your final pull request of this project:

* Programming Language: the one that fits you better.
* Database: ScyllaDB
* REPL: Application that will Read, Eval, Print and Loop through your commands.
	* Command: Create a todo;
	* Command: Read a todo from database;
	* Command: Update a todo from database;
	* Command: Delete a todo from database;
	* Command: Maximum Throughput;
	* Command: Migrate CQL files to the Keyspace;
	* Command: Exit App;

<p align="center">
    <img src="https://dev-to-uploads.s3.amazonaws.com/uploads/articles/e8vz26sagfakaryzmrbg.png">
</p>

## 3. Scylla Cloud First Steps

This section will show the basics about Scylla Cloud and also how to create and connect on your first cluster.

### 3.1 Creating a cluster

On [Scylla Cloud](https://cloud.scylladb.com/), navigate to the `My Clusters` page and click on `Create New Cluster`
card. Enter the information below:

* Cluster Name: monstrously-todo-cluster;
* Provider: AWS;
* Which Account: ScyllaDB Account;
* Region: Pick the closest to you.

<p align="center">
  <img src="https://dev-to-uploads.s3.amazonaws.com/uploads/articles/zk51uisfgcnf5ighxsr9.png" alt="">
</p>

Scylla Cloud provides you a free tier cluster, so make sure to choose the *Evaluation* cluster.

<p align="center">
  <img src="https://dev-to-uploads.s3.amazonaws.com/uploads/articles/zbem992u4kdd6ebw2bpx.png" alt="">
</p>

Add your IP address on the next step and launch your instance. The process take a few minutes, so use this time to fill
your bottle of water ;)

### 3.2 Connect to your Cloud Cluster

On your cluster's information page, locate your username, password and node IP addresses.
Run the command below and connect to your cluster:

```shell
docker run -it --rm --entrypoint cqlsh scylladb/scylla -u [USERNAME] -p [PASSWORD] [NODE IP]
```

<p align="center">
    <img src="https://dev-to-uploads.s3.amazonaws.com/uploads/articles/6e96us8fgpbys36xa82q.gif">
</p>

## 4. Building the App

### 4.1 REPL Runner

This will be the base of your app and it will be a simple `while(true)` receiving the user's input (STDIN) inside it.

#### Scenario

- **Scenario:** You as a user
- **When:** run your main project file
- **Then:** open a "chat" to type commands

#### Examples

```php
// lang: php
while (true) {
	echo "danielhe4rt:";
	$input = fgets(STDIN);

	if ($input == '!somecommand') {
		// do stuff
	}
}
```

### 4.2 Command: Exit

Finish the REPL process and close the application.

* **Scenario:** You as an user
* **When:** type the command "!exit"
* **Then:** finish the application with the output "See ya!"

#### Examples

```php
while (true) {
	echo "danielhe4rt:";
	$input = fgets(STDIN);

	if ($input == '!exit') {
		echo "See ya!";
		break;
	}
}
```


### 4.2 Command: Create a TODO

Start receiving the to-do inputs and persist it on ScyllaDB.
> Look at your driver docs the class/method that handles the creation of Uuids

Query:
```sql
INSERT INTO todos (id, description, completed, created_at) VALUES (7dac16d5-cf44-4980-96e8-dd862a366109, 'your-task', false, '2021-06-01 01:00:00.000+0000');
```

* **Scenario:** You as a user
* **When:** type the command "!todo new"
* **Then:** prompt a message 'What is this task about?' and store the answer.
* **Then:** Insert the data inside your 'todos' table with *id*, *completed* and *created_at* fields.
* **Then:** Output the message "New task added!"

### 4.3 Command: List Todos

Display to user the last N todo items from your table.
> To understand more about "per partition limit", please go to [ this link](https://docs.scylladb.com/stable/cql/dml.html#limiting-results).

Query:
```sql
select * from todos PER PARTITION LIMIT 1 LIMIT 30;
```

* **Scenario:** You as a user
* **When:** type the command "!list"
* **Then:** Output a table with last 30 todo items
* **Then:** Output the message "Done."

### 4.4 Command: List Todos

Display to user the last N todo items from your table.
> To understand more about "per partition limit", please go to [ this link](https://docs.scylladb.com/stable/cql/dml.html#limiting-results).

Query:
```sql
select * from todos WHERE completed = false PER PARTITION LIMIT 1 LIMIT 30;

UPDATE todos SET completed_at = '2023-03-16 16:25:55.000' WHERE id = 7dac16d5-cf44-4980-96e8-dd862a366109 AND created_at = '2021-06-03 01:00:00.000'
```

* **Scenario:** You as a user
* **When:** type the command "!complete task"
* **Then:** Output a table with last 30 todo items that is not finished yet with the index from your array at the beginning
* **Then:** prompt a message "Select the todo to finish: "
* **Then:** Find at database the related record and update it.
* **Then:** Output the message "Done."

### 4.5 Command: Delete Todos

Display to user the last N todo items from your table.
> To understand more about "per partition limit", please go to [ this link](https://docs.scylladb.com/stable/cql/dml.html#limiting-results).

Query:
```sql
select * from todos WHERE completed = false PER PARTITION LIMIT 1 LIMIT 30;

DELETE FROM todos WHERE id = 7dac16d5-cf44-4980-96e8-dd862a366109
```

* **Scenario:** You as a user
* **When:** type the command "!delete task"
* **Then:** Output a table with last 30 todo items that is not finished yet with the index from your array at the beginning
* **Then:** prompt a message "Select the todo to finish: "
* **Then:** Find at database the related record and delete it.
* **Then:** Output the message "Done."

### 4.5 Command: Stress Testing

Create a specific amount of finished todos with pre-set data.
> You can look for functions that generates random data or libs like 'Faker' to handle it for you.

Query:
```sql
INSERT INTO todos (id, description, completed, completed_at created_at) VALUES (7dac16d5-cf44-4980-96e8-dd862a366109, 'your-task', false, '2021-06-01 01:00:00.000+0000' ,'2021-06-01 01:00:00.000+0000');
```

* **Scenario:** You as a user
* **When:** type the command "!delete task"
* **Then:** Output a table with last 30 todo items that is not finished yet with the index from your array at the beginning
* **Then:** prompt a message "Select the todo to finish: "
* **Then:** Find at database the related record and delete it.
* **Then:** Output the message "Done."

## 5. Contributing

If you want to contribute with a new language-specific implementation of this project, feel free to fork and open a pull request.

Just remember to envolve your amazing code with an amazing README that will help other developers to run it in their local environment!

```
~
├── README.md <- You are HERE.
├── php
│   └── README.md
└── ruby
    └── README.MD
```


Check the available [drivers here](https://docs.scylladb.com/stable/using-scylla/drivers/cql-drivers/index.html) and if you want to contribute in more than one project, feel free to do it!
