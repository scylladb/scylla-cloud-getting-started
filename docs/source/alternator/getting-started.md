# Getting Started with ScyllaDB Cloud: A sample Media Player App with Alternator


## 1. Introduction

This guide will show you how to create a Cluster into ScyllaDB Cloud, create an Media Player app from scratch and configure it
to use Scylla as the backend datastore. It'll walk you through all the stages
of the development process, from gathering requirements to building and running
the application.

As an example, you will use an application called Media Player. Media Player stores all the tracks
that you want to listen and count how many times you listened to. The application consists of three parts:
-   Validate if you have the Keyspace and Table needed to start storing your tracks/songs;
-   Access to an REPL with commands to store/list/delete songs;
-   Run a simple stresser into ScyllaDB Cloud.

## 2. Requirements

Before start your first project with Scylla Alternator API, we need to setup your environment.

### 2.1 Create a ScyllaDB Cloud account

Before you start coding the project, you should create an account at [ScyllaDB Cloud](https://cloud.scylladb.com) or login if you already have one.

![ScyllaDB Cloud Registration Page](/_static/img/alternator/getting-started/scylla-registration-page.png)

### 2.2 Create a Sandbox Cluster 

After create and login into your ScyllaDB Cloud account, click on "New Cluster" tab. There, you should:

- Give your cluster a cool name for this project;
- Select "ScyllaDB Alternator - DynamoDB API " on the **Scylla Version**;
- Select the nearest region for your cluster.


![ScyllaDB Creating a new instance](/_static/img/alternator/getting-started/scylladb-1.png)

After that, check the **"t3.micro"** model (Sandbox) and click in "Next".

![ScyllaDB Cloud Registration Page](/_static/img/alternator/getting-started/scylladb-2.png)


On the network tab, just make sure that your IP Address is correct and click in "Launch Cluster".


![Cluster creating in progress page](/_static/img/alternator/getting-started/scylladb-creating-cluster.png)

## 3. Connecting into your cluster

Now that you have the ScyllaDB Alternator running, you will see a panel like the picture below. It will be a three node cluster.

![Cluster Overview page](/_static/img/alternator/getting-started/scylladb-cluster-overview.png)

As you can see on the first node, the ip "18.231.92.93" will be used to connect on the "Endpoint URL" at any DynamoDB SDK or API. 

Examples: 

```python
# Python Example
import boto3

alternator_client = boto3.resource('dynamodb',endpoint_url='http://18.231.92.93:8000',
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

Now you are able to send requests to your ScyllaDB Alternator instance! Select the SDK on the language of your preference and happy coding!

## 4. Next Steps

Now that you have everything ready, let's move to the project Design and Data Model with DynamoDB spectations!

