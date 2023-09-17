namespace MediaPlayer;

public static class Constants
{
    public static string CreateKeyspaceIfDoesntExistQuery => "CREATE KEYSPACE prod_media_player WITH replication = { 'class': 'NetworkTopologyStrategy', 'replication_factor': '3'} AND durable_writes = true";
    public static string CheckKeyspaceQuery => "SELECT keyspace_name FROM system_schema.keyspaces WHERE keyspace_name=?";
    public static string CreateTableSongExistQuery => "CREATE TABLE prod_media_player.songs (id uuid, title text, album text, artist text, created_at timestamp, PRIMARY KEY (id, created_at))";
    public static string CreateTableSongCounterQuery => "CREATE TABLE prod_media_player.song_counter (song_id uuid, times_played counter, PRIMARY KEY (song_id))";
    public static string CheckTableQuery => "select keyspace_name, table_name from system_schema.tables where keyspace_name = ? AND table_name = ?";
    public static string CreateSongQuery => "INSERT INTO prod_media_player.songs (id,title,artist,album,created_at) VALUES (?,?,?,?,?)";
    public static string DeleteSongQuery => "DELETE FROM prod_media_player.songs WHERE id = ?";
    public static string UpdateSongCounterQuery =>
        "UPDATE prod_media_player.added_songs_counter SET amount = amount + 1 WHERE id = 1";
    public static string ListSongsQuery => "SELECT id, title, album, artist, created_at FROM prod_media_player.songs LIMIT 10";
}