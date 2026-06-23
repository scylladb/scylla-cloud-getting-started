# Getting Started with ScyllaDB Cloud + DynamoDB Compatible API: A sample Media Player App with Alternator


## 1. Introduction

This guide will show you how to create a cluster in ScyllaDB Cloud, create a Media Player app from scratch, and configure it
to use Scylla as the backend datastore. It'll walk you through all the stages
of the development process, from gathering requirements to building and running
the application.

As an example, you will use an application called Media Player. Media Player stores all the tracks
you want to listen to and counts how many times you have listened to them. The application consists of three parts:
-   Validate if you have the Keyspace and Table needed to start storing your tracks/songs;
-   Access a REPL with commands to store/list/delete songs;
-   Run a simple stressor against ScyllaDB Cloud.

## 2. Requirements

Before starting your first project with the ScyllaDB Alternator API, we need to set up your environment.

### 2.1 Create a ScyllaDB Cloud account

Before you start coding the project, you should create an account at [ScyllaDB Cloud](https://cloud.scylladb.com) or login if you already have one.

![ScyllaDB Cloud Registration Page](/_static/img/alternator/getting-started/scylla-registration-page.png)

### 2.2 Create a Sandbox Cluster 

After creating and logging in to your ScyllaDB Cloud account, click on the "New Cluster" tab. There, you should:

- Give your cluster a name
- Select "Standard" cluster type for small tests, otherwise select "X Cloud" for maximum elasticity
- Select "Amazon DynamoDB API compatible" in the **ScyllaDB API** section;
- Select the nearest region for your cluster


![ScyllaDB Creating a new instance](/_static/img/alternator/getting-started/scylladb-1.png)

After that, check the **"t3.micro"** model (Sandbox, only available in Standard clusters) and click "Next".

![ScyllaDB Cloud Registration Page](/_static/img/alternator/getting-started/scylladb-2.png)


On the network tab, make sure your IP address is correct and click "Launch Cluster".


![Cluster creating in progress page](/_static/img/alternator/getting-started/scylladb-creating-cluster.png)

## 3. Connecting to your cluster

Now that you have the ScyllaDB Alternator running, you will see a panel like the picture below. It will be a three node cluster.

![Cluster Overview page](/_static/img/alternator/getting-started/scylladb-cluster-overview.png)

As you can see on the first node, the IP "100.51.91.97" will be used to connect to the "Endpoint URL" in any DynamoDB SDK or API. 

Examples: 

```python
# Python Example
import boto3

alternator_client = boto3.resource('dynamodb',endpoint_url='http://localhost:8000',
                region_name='None', aws_access_key_id='None', aws_secret_access_key='None')
```

```php
// PHP with AWS SDK
use Aws\DynamoDb\DynamoDbClient;

$alternatorClient = new DynamoDbClient([  
    'endpoint' => "18.231.92.93:8000", // Alternator
    'credentials' => ['key' => 'None', 'secret' => 'None'],  
    'region' => 'None'  
]);
```

Now you are able to send requests to your ScyllaDB Alternator instance. Select the SDK for the language of your choice and start building!

## 4. Next Steps

Now that you have everything ready, let's move to the project Design and Data Model with DynamoDB spectations!

