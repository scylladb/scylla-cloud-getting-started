<?php

namespace App;

use Aws\DynamoDb\DynamoDbClient;

class Migration
{

    public static function run(DynamoDbClient $client): void
    {
        $availableTableNames = $client->listTables()->toArray()['TableNames'];

        if (in_array('songs', $availableTableNames)) {
            return;
        }

        $client->createTable(self::createTableQuery());

        $client->waitUntil('TableExists', [
            'TableName' => 'songs',
            '@waiter' => [
                'delay' => 5,
                'maxAttempts' => 3
            ]
        ]);
    }

    private static function createTableQuery(): array
    {
        return [
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
    }
}