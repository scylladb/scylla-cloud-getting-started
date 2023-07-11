from cassandra.cluster import Cluster, BatchStatement, ConsistencyLevel, SimpleStatement
from cassandra.auth import PlainTextAuthProvider
import uuid
from datetime import datetime
from helpers import migrate
from environment import enviroment


cluster = Cluster(
    contact_points=enviroment['contact_points'],
    auth_provider=PlainTextAuthProvider(username=enviroment['username'], password=enviroment['password'])
)

session = cluster.connect()
print('-------------------------')
migrate(session)
    
session.set_keyspace(enviroment['keyspace'])

print('-------------------------')
print('Admin: Welcome to MediaPlayer Metrics')


while True:
    
    command = input("User: ")
    
    if command == "!new":
        songName = input("Which song you want to add? ")
        artistName = input("From which artist? ")
        albumName = input("From which album? ")
        releaseYear = input("Release Year? ")
        
        query = session.prepare("INSERT INTO songs (id, title, album, artist, release_year, created_at) VALUES (?,?,?,?,?,?)")
        
        session.execute(query, (
            uuid.uuid4(),
            songName,
            albumName,
            artistName,
            int(releaseYear),
            datetime.now()
        ));
        
        print(f'Admin: song {songName} added successfuly!')
    
    if command == "!delete":
        print('Admin: Listing all the songs registered so far...')
        listSongs = session.execute("SELECT * FROM songs")
        
        enumSongList = list(enumerate(listSongs))
        print('------------------------')
        for rowIndex, song in enumSongList:
            print(f"Index: {rowIndex} | Title: {song.title} | Album: {song.album} | Year: {song.release_year}")
        
        print('------------------------')
        songIndex = int(input('Select an index: '))
        
        songToDelete = enumSongList[songIndex]
        session.execute('DELETE FROM songs WHERE id = %s', ([songToDelete[1].id]))
        
        print(f'Admin: song {songName} deleted successfuly!')
        
    if command == "!listen":
        print('Admin: Listing all the songs registered so far...')
        listSongs = session.execute("SELECT * FROM songs")
        
        enumSongList = list(enumerate(listSongs))
        print('------------------------')
        for rowIndex, song in enumSongList:
            print(f"Index: {rowIndex} | Title: {song.title} | Album: {song.album} | Year: {song.release_year}")
        
        print('------------------------')
        songIndex = int(input('Select an index: '))
        
        selectedSong = enumSongList[songIndex]
        
        session.execute_async(session.prepare('UPDATE played_songs_counter SET times_played = times_played + 1 WHERE song_id = ?'), [
            selectedSong[1].id
        ])
        session.execute_async(session.prepare('INSERT INTO recently_played_songs (song_id, listened_at) VALUES (?, ?)'), [
            selectedSong[1].id,
            datetime.now()
        ]);
        
        print("Admin: Song incremented sucessfuly!")
    
    if command == "!stress":
        print('Looping through all the songs registered so far...')
        print('Incrementing "played_songs_counter" table...')
        print('Adding recent played songs to "recently_played_songs" table...')
        print('Check your ScyllaDB Cloud Monitoring to check your query status!')
        songsCounterQuery = session.prepare('UPDATE played_songs_counter SET times_played = times_played + 1 WHERE song_id = ?')
        recentlyPlayedSongsQuery = session.prepare('INSERT INTO recently_played_songs (song_id, listened_at) VALUES (?, ?)')
        
        listSongs = session.execute("SELECT * FROM songs")
        enumSongList = list(enumerate(listSongs))
        while True:
            for rowIndex, song in enumSongList:
                session.execute_async(songsCounterQuery, [song.id])
                session.execute_async(recentlyPlayedSongsQuery, [song.id, datetime.now()])