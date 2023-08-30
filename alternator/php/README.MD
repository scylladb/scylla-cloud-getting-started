# ScyllaDB Cloud Media Player Alternator - PHP

Project to store songs that you like to listen daily and keep track of them in a shape of a CLI!

## Prerequisites

* [PHP 8.2](https://php.net/)
* [Composer](https://getcomposer.org/download/)

## Running the project

Clone the repository into your machine:

```sh 
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/alternator/php
```

Install the project dependencies and run the project:

```sh
composer install
```

Go to the 'src/App.php' and change the 'endpoint url' to your Alternator API url. 

After that run the project with: 

````sh
php app.php
````

> Replace the variables with your cluster information

## Available Commands

Check which commands are currently available on this sample:

| Command      | Description                                                                        |
|--------------|------------------------------------------------------------------------------------|
| !insert-song | Add a new song to your liked songs list;                                           |
| !list-songs  | List all songs on the project;                                                     |
| !delete-song | Delete a specific song of the project;                                             |

## DynamoDB Queries

All the CQL queries used on the project

```php

$createTable = [
    'TableName' => 'songs',
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
];

$insertSong = [
    'TableName' => 'songs',
    'Item' => [
        'id' => ['S' => 'string'],
        'created_at' => ['S' => 'string'],
        'title' => ['S' => 'string'],
        'album' => ['S' => 'string'],
        'artist' => ['S' => 'string'],
    ]
];

$listSongs = [
    'TableName' => 'songs'
];

$deleteSongs = [
    'TableName' => 'songs',
    'Key' => [
        'id' => ['S' => 'string'],
        'created_at' => ['S' => 'string'],
    ],
];

```