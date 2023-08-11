# Getting Started with ScyllaDB Cloud: A sample Media Player App


## Introduction

This guide will show you how to create a Cluster into ScyllaDB Cloud, create an Media Player app from scratch and configure it
to use Scylla as the backend datastore. It'll walk you through all the stages
of the development process, from gathering requirements to building and running
the application.

As an example, you will use an application called Media Player. Media Player stores all the tracks
that you want to listen and count how many times you listened to. The application consists of three parts:
-   Validate if you have the Keyspace and Table needed to start storing your tracks/songs;
-   Access to an REPL with commands to store/list/delete songs;
-   Run a simple stresser into ScyllaDB Cloud.

## Requirements

### Prerequisites for Deploying the Application

The example application uses ScyllaDB Cloud to run a three-node ScyllaDB cluster. You can claim your free Scylla Cloud account [here](https://scylladb.com/cloud).


### Performance Requirements

The application has two performance-related parts: sensors that write to
the database (throughput sensitive) and a backend dashboard that reads from
the database (latency sensitive). 

* This example assumes 99% writes (songs) and 1% reads (backend dashboard).  
* SLA:
  - Writes: throughput of 100K operations per second.
  - Reads: latency of up to 10 milliseconds for the
    [99th percentile](https://engineering.linkedin.com/performance/who-moved-my-99th-percentile-latency).
* The application requires high availability and fault tolerance. Even if a
ScyllaDB node goes down or becomes unavailable, the cluster is expected to
remain available and continue to provide service. You can learn more about
Scylla high availability in [this lesson](https://university.scylladb.com/courses/scylla-essentials-overview/lessons/high-availability/). 


### Additional Resources

-   [Scylla Essentials](https://university.scylladb.com/courses/scylla-essentials-overview/) course on Scylla University. It provides an introduction to Scylla and explains the basics.
-   [Data Modeling and Application Development](https://university.scylladb.com/courses/data-modeling/) course on Scylla University. It explains basic and advanced data modeling techniques, including information on workflow application, query analysis, denormalization, and other NoSQL data modeling topics.
-   [Scylla Documentation](https://docs.scylladb.com/)
-   Scylla users [slack channel](http://slack.scylladb.com/)

