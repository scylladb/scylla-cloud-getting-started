enviroment = {
    'contact_points': [
        "node-0.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
        "node-1.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud", 
        "node-2.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud"
    ],
    'username': 'scylla',
    'password': 'r4GnOL2QSDi1wqF',
    'keyspace': 'prod_media_player',
    'tables': {
        'songs': """
            CREATE TABLE prod_media_player.songs (
                id uuid,
                title text,
                album text,
                artist text,
                release_year int,
                created_at timestamp,
                PRIMARY KEY (id, created_at)
            )
        """,
        'recently_played_songs': """
            CREATE TABLE prod_media_player.recently_played_songs (
                song_id uuid,
                listened_at timestamp,
                PRIMARY KEY (song_id, listened_at)
            )
        """,
        'played_songs_counter': """
            CREATE TABLE prod_media_player.played_songs_counter (
                song_id uuid,
                times_played counter,
                PRIMARY KEY (song_id)
            )
        """
    },
}