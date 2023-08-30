def migrate(alternator_client) -> None:
    print("Verifying Migrations...")

    tables = alternator_client.tables.all()
    available_tables = []
    for table in tables:
        available_tables.append(table.name)
    
    
    
    if 'songs' not in available_tables:
        table = alternator_client.create_table(
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