import boto3
import uuid
from helpers import migrate;
from datetime import datetime

dynamodb = boto3.resource('dynamodb', endpoint_url='http://18.231.92.93:8000',
                          region_name='None', aws_access_key_id='None', aws_secret_access_key='None')
migrate(dynamodb)

table = dynamodb.Table('songs');

while True: 
    
    command = input("User: ")
    
    if command == "!new":
        songName = input("Which song you want to add? ")
        artistName = input("From which artist? ")
        albumName = input("From which album? ")
        
        table.put_item(Item={
            'id': str(uuid.uuid4()),    
            'created_at': str(datetime.now()),
            'title': songName,
            'album': albumName,
            'arist': artistName,
        })
        
        
        print(f'Admin: song {songName} added successfuly!')
        
    if command == "!list":
        print('Admin: Listing all the songs registered so far...')
        listSongs = table.scan()
    
        enumSongList = list(enumerate(listSongs['Items']))
        print('------------------------')
        for rowIndex, song in enumSongList:
            print(f"Index: {rowIndex} | Title: {song['title']} | Album: {song['album']} ")
        
        print('------------------------')
        
        
    if command == "!delete":
        print('Admin: Listing all the songs registered so far...')
        listSongs = table.scan()
    
        enumSongList = list(enumerate(listSongs['Items']))
        print('------------------------')
        for rowIndex, song in enumSongList:
            print(f"Index: {rowIndex} | Title: {song['title']} | Album: {song['album']} ")
        
        print('------------------------')
        songIndex = int(input('Select an index: '))
        
        songToDelete = enumSongList[songIndex]
        
        
        table.delete_item(Key={
            'id': songToDelete[1]['id'],
            'created_at': songToDelete[1]['created_at']
        })
        
        print(f'Admin: song ', songToDelete[1]['title'], ' deleted successfuly!')