# ScyllaDB Alternator: PHP Example

Using the "PHP AWS SDK" we can easily connect our project on any ScyllaDB instance (Local/Cloud) that is configured with
Alternator flag.

## Quick Start

If you're starting a new project, make sure to require the SDK package in your application. 

````shell
composer require aws/aws-sdk-php
````

You can instantiate a new DynamoDB Client by using: 
````php
use Aws\DynamoDb\DynamoDbClient;

$alternatorClient = new DynamoDbClient([
    'endpoint' => 'http://localhost:8000', // ScyllaDB Alternator LocalHost URL
    'credentials' => ['key' => 'None', 'secret' => 'None'],
    'region' => 'None'
]);
````

## Queries 
Here's the DynamoDB queries used on this project so far:

### Creating a Table

````php
use Aws\DynamoDb\DynamoDbClient;

$alternatorClient = new DynamoDbClient([
    'endpoint' => 'http://localhost:8000', // ScyllaDB Alternator LocalHost URL
    'credentials' => ['key' => 'None', 'secret' => 'None'],
    'region' => 'None'
]);

$alternatorClient->createTable([
    'TableName' => 'weather',
    'KeySchema' => [
        ['AttributeName' => 'city_name', 'KeyType' => 'HASH'],
        ['AttributeName' => 'ts', 'KeyType' => 'RANGE'],
    ],
    'AttributeDefinitions' => [
        ['AttributeName' => 'city_name', 'AttributeType' => 'S'],
        ['AttributeName' => 'ts', 'AttributeType' => 'S'],
        ['AttributeName' => 'temperature', 'AttributeType' => 'M'],
        ['AttributeName' => 'ivu', 'AttributeType' => 'N'],
        ['AttributeName' => 'climate_conditions', 'AttributeType' => 'S'],

    ],
    'ProvisionedThroughput' => [
        'ReadCapacityUnits' => 10,
        'WriteCapacityUnits' => 10
    ]
]);

````

### Adding Items

````php
use Aws\DynamoDb\DynamoDbClient;

$alternatorClient = new DynamoDbClient([
    'endpoint' => 'http://localhost:8000', // ScyllaDB Alternator LocalHost URL
    'credentials' => ['key' => 'None', 'secret' => 'None'],
    'region' => 'None'
]);

$alternatorClient->putItem('PutRequest' => [
    'TableItem' => 'weather',
    'Item' => [
        'city_name' => ['S' => 'São Paulo'],
        'ts' => ['S' => '2023-08-16'],
        'temperature' => ['M' => [
            'mininum' => ['N' => 15],
            'maximum' => ['N' => 31],
            'average' => ['N' => 24],
        ]],
        'uvi' => ['N' => 10],
        'climate_conditions' => ['S' => 'Sunny']
    ]
]);

````
