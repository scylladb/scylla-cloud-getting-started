# ScyllaDB Alternator: PHP Example

Using the "PHP AWS SDK" we can easily connect our project on any ScyllaDB instance (Local/Cloud) that is configured with Alternator flag.

## Quick Start

Create a new folder

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

$alternatorClient->createTable(['TableName' => 'songs',
    'KeySchema' => [
        ['AttributeName' => 'id', 'KeyType' => 'HASH'],
        ['AttributeName' => 'created_at', 'KeyType' => 'RANGE'],
    ],
    'AttributeDefinitions' => [
        ['AttributeName' => 'id', 'AttributeType' => 'S'],
        ['AttributeName' => 'created_at', 'AttributeType' => 'S'],
        ['AttributeName' => 'title', 'AttributeType' => 'S'],
        ['AttributeName' => 'artist', 'AttributeType' => 'S'],
        ['AttributeName' => 'album', 'AttributeType' => 'S'],
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
    'TableName' => 'songs',
    'Item' => [
        'id' => ['S' => 'string'],
        'created_at' => ['S' => 'string'],
        'title' => ['S' => 'string'],
        'album' => ['S' => 'string'],
        'artist' => ['S' => 'string'],
    ]
]);

````

### Listing Items

```php
use Aws\DynamoDb\DynamoDbClient;
use Aws\DynamoDb\Marshaler;

$marshaler = new Marshaler();
$alternatorClient = new DynamoDbClient([
    'endpoint' => 'http://localhost:8000', // ScyllaDB Alternator LocalHost URL
    'credentials' => ['key' => 'None', 'secret' => 'None'],
    'region' => 'None'
]);

$results = $client->scan(['TableName' => 'songs']);


foreach ($results['Items'] as $item) {
    $parsedItem = $marshaler->unmarshalItem($item);
    var_dump($parsedItem['title'])
}

```

### Deleting Items


```php
use Aws\DynamoDb\DynamoDbClient;
use Aws\DynamoDb\Marshaler;

$marshaler = new Marshaler();
$alternatorClient = new DynamoDbClient([
    'endpoint' => 'http://localhost:8000', // ScyllaDB Alternator LocalHost URL
    'credentials' => ['key' => 'None', 'secret' => 'None'],
    'region' => 'None'
]);

$client->deleteItem([
    'TableName' => 'songs',
    'Key' => [
        'id' => ['S' => 'some-uuid-here'],
        'created_at' => ['S' => 'Y-m-d H:i:s'],
    ],
]);

```