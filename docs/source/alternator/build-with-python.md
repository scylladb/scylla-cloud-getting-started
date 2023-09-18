# ScyllaDB Alternator: PHP Example

Using the "Python AWS SDK" we can easily connect our project on any ScyllaDB instance (Local/Cloud) that is configured with Alternator flag.

## Quick Start

Create a new folder

If you're starting a new project, make sure to require the SDK package in your application. 

````shell
sudo pip install --upgrade boto3
````

You can instantiate a new DynamoDB Client by using: 
````python
import boto3

alternator = boto3.resource('dynamodb',endpoint_url='http://localhost:8000',
                region_name='None', aws_access_key_id='None', aws_secret_access_key='None')

````

## Queries 
Here's the DynamoDB queries used on this project so far:

### Creating a Table

````python
import boto3

alternator = boto3.resource('dynamodb',endpoint_url='http://localhost:8000',
                region_name='None', aws_access_key_id='None', aws_secret_access_key='None')

table = alternator.create_table(
    TableName="songs",
    KeySchema=[
        {'AttributeName': 'id', 'KeyType': 'HASH'},
        {'AttributeName': 'created_at', 'KeyType': 'RANGE'},
    ],
    AttributeDefinitions=[
        {'AttributeName': 'id', 'AttributeType': 'S'},
        {'AttributeName': 'created_at', 'AttributeType': 'S'},
        {'AttributeName': 'title', 'AttributeType': 'S'},
        {'AttributeName': 'artist', 'AttributeType': 'S'},
        {'AttributeName': 'album', 'AttributeType': 'S'},
    ],
    ProvisionedThroughput={'ReadCapacityUnits': 10, 'WriteCapacityUnits': 10}
);
table.wait_until_exists()

table.put_item(Item={
    'id': str(uuid.uuid4()),    
    'created_at': str(datetime.now()),
    'title': 'Song 1',
    'album': 'Album 1',
    'artist': 'artistName',
})

````

### Adding Items

```python
alternator = boto3.resource('dynamodb',endpoint_url='http://localhost:8000',
                region_name='None', aws_access_key_id='None', aws_secret_access_key='None')

table = alternator.Table('songs')

table.put_item(Item={
    'id': str(uuid.uuid4()),    
    'created_at': str(datetime.now()),
    'title': 'Song 1',
    'album': 'Album 1',
    'artist': 'artistName',
})

```

### Listing Items

```python

alternator = boto3.resource('dynamodb',endpoint_url='http://localhost:8000',
                region_name='None', aws_access_key_id='None', aws_secret_access_key='None')

table = alternator.Table('songs')

listSongs = table.scan()
enumSongList = list(enumerate(listSongs['Items']))

for rowIndex, song in enumSongList:
    print(f"Index: {rowIndex} | Title: {song['title']} | Album: {song['album']} ")

```

### Delete Item

```python

alternator = boto3.resource('dynamodb',endpoint_url='http://localhost:8000',
                region_name='None', aws_access_key_id='None', aws_secret_access_key='None')

table = alternator.Table('songs')
listSongs = table.scan()

table.delete_item(Key={
    'id': 'id',
    'created_at': 'y-m-d H:i:s'
})
```