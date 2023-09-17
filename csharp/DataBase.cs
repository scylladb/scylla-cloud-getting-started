using Cassandra;

namespace MediaPlayer;

public class DataBase : IDisposable
{
    private readonly ISession _session;
    public DataBase(string[] credentials)
    {
        var session = Connect(credentials);
        _session = session;
    }

    public async Task Add(Song song)
    {
        string query = Constants.CreateSongQuery;
        var ps = await _session.PrepareAsync(query);
        var statement = ps.Bind(song.Id, song.Title, song.Artist, song.Album, song.CreatedAt);

        await _session.ExecuteAsync(statement);
    }

    public async Task<List<Song>> ListSongs()
    {
        List<Song> songs = new();
        string query = Constants.ListSongsQuery;
        var ps = await _session.PrepareAsync(query);
        var statement = ps.Bind();
        var result = await _session.ExecuteAsync(statement);

        var rows = result.GetRows().ToList();

        foreach (var row in rows)
        {
            Guid id = row.GetValue<Guid>("id");
            string title = row.GetValue<string>("title");
            string album = row.GetValue<string>("album");
            string artist = row.GetValue<string>("artist");
            DateTime createdAt = row.GetValue<DateTime>("created_at");

            var song = new Song()
            {
                Id = id,
                CreatedAt = createdAt,
                Album = album,
                Artist = artist,
                Title = title
            };
            songs.Add(song);
        }
        
        return songs;
    }

    public async Task Delete(Song song)
    {
        string query = Constants.DeleteSongQuery;
        var ps = await _session.PrepareAsync(query);
        var statement = ps.Bind(song.Id);
        await _session.ExecuteAsync(statement);
    }

    private ISession Connect(string[] credentials)
    {
        try
        {
            var username = credentials[0];
            var password = credentials[1];
            var nodeOne = credentials[2];
            var nodeTwo = credentials[3];
            var nodeThree = credentials[4];

            var cluster = Cluster.Builder()
                .AddContactPoints(nodeOne)
                .AddContactPoints(nodeTwo)
                .AddContactPoints(nodeThree)
                .WithQueryTimeout(5000)
                .WithCredentials(username, password)
                .Build();

            var session = cluster.Connect();
            return session;
        }
        catch
        {
            Console.WriteLine("Connection Refused. Check your credentials and your IP linked on the ScyllaDb Cloud.");
            return null;
        }
    }

    public async Task Migrate()
    {
        Console.WriteLine("-----------------------------------");
        Console.WriteLine("->.......Verifying Database.......<-");
        await CreateKeySpace();
        Console.WriteLine("->........Keyspace setted.........<-");
        await CreateTables();
        Console.WriteLine("->.........Tables setted..........<-");
        Console.WriteLine("------------------------------------");

    }

    private async Task CreateKeySpace()
    {
        var keyspaceQuery = Constants.CheckKeyspaceQuery;
        var ps = await _session.PrepareAsync(keyspaceQuery);
        var statement = ps.Bind("prod_media_player");

        var result = await _session.ExecuteAsync(statement);

        var rows = result.GetRows().ToList();
        if(rows.Count == 0)
        {
            var createKeyspaceQuery = Constants.CreateKeyspaceIfDoesntExistQuery;
            var preparedStatement = await _session.PrepareAsync(createKeyspaceQuery);
            var st = preparedStatement.Bind();
            await _session.ExecuteAsync(st);
        }
    }

    private async Task CreateTables()
    {
        var keyValuePairs = new Dictionary<string, string>()
        {
            { "songs", Constants.CreateTableSongExistQuery },
            { "song_counter", Constants.CreateTableSongCounterQuery }
        };

        foreach (var kvp in keyValuePairs)
        {
            var checkTableQuery = Constants.CheckTableQuery;
            var ps = await _session.PrepareAsync(checkTableQuery);
            
            var statement = ps.Bind("prod_media_player", kvp.Key);

            var result = await _session.ExecuteAsync(statement);

            var rows = result
                .GetRows()
                .ToList();
            
            if(rows.Count == 0)
            {
                var preparedStatement = await _session.PrepareAsync(kvp.Value);
                var st = preparedStatement.Bind();
                await _session.ExecuteAsync(st);
            }    
        }
    }
    public void Dispose()
    {
        _session.Dispose();
    }
}