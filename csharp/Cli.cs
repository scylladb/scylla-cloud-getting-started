namespace MediaPlayer;

public class Cli
{
    private readonly DataBase _dataBase;

    public Cli(string[] args)
    {
        _dataBase = new DataBase(args);
    }

    public async Task Intro()
    {
        Console.WriteLine("------------------------------------");
        Console.WriteLine("- ScyllaDB Cloud Rust Media Player -");
        Console.WriteLine("------------------------------------");
        Console.WriteLine("-    Leave a star on the repo      -");
        Console.WriteLine("-     https://bit.ly/scy-gh        -");
        Console.WriteLine("------------------------------------");
        await _dataBase.Migrate();
        Console.WriteLine("-----------------------------------");
        
    }

    private void DisplayHint()
    {
        Console.WriteLine("------------------------------------");
        Console.WriteLine("Here some possibilities");
        Console.WriteLine("  !add - add new song");
        Console.WriteLine("  !list - list all songs");
        Console.WriteLine("  !delete - delete a specific song");
        Console.WriteLine("  !stress - stress testing with mocked data");
        Console.WriteLine("------------------------------------");
    }

    public async Task Start()
    {
        try
        {
            bool lifeTimeCli = true;
            DisplayHint();
        
            while (lifeTimeCli)
            {
                var command = GetCommand();
                var _ = command switch
                {
                    "!add" => AddSong(),
                    "!list" => ListSongs(),
                    "!stress" => Stress(),
                    "!q" => Exit(),
                    _ => Start()
                };
                DisplayHint();
            }
        }
        catch(Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        
        // loop {
        //     let command = get_command();
        //
        //     let _ = match command.as_str().trim() {
        //         "!add" => commands::add_song(&mut database).await,
        //         "!list" => commands::list_songs(&database).await,
        //         "!delete" => commands::delete_song(&mut database).await,
        //         "!stress" => commands::stress(Arc::new(Database::new(&args).await)).await,
        //         "!q" => panic!("See ya!"),
        //         _ => Ok(()),
        //     };
        //     display_help();
        // }
    }

    private string GetCommand()
    {
        Console.Write("Type any command: ");
        var command = Console.ReadLine();

        if (string.IsNullOrEmpty(command))
        {
            DisplayHint();
            GetCommand();
        }

        return command;
    }

    private async Task AddSong()
    {
        Console.Write($"Song name: ");
        string title = Console.ReadLine();
        
        Console.Write($"Album: ");
        string album = Console.ReadLine();
        
        Console.Write($"Artist: ");
        string artist = Console.ReadLine();
        
        var song = new Song()
        {
            Id = Guid.NewGuid(),
            Album = album,
            Artist = artist,
            Title = title
        };

        Console.WriteLine($"Song {song.Title} from artist {song.Artist} Added!");

        await _dataBase.Add(song);
    }

    private async Task ListSongs()
    {
        Console.WriteLine($"Here is the songs added so far: ");
        Console.WriteLine($"-----------------------------------");
        
        var songs = await _dataBase.ListSongs();

        foreach (var song in songs)
        {
            Console.WriteLine($"ID: {song.Id} | Song: {song.Title} | Album: {song.Album} | Artist: {song.Artist} | Created At: {song.CreatedAt}");
        }
        Console.WriteLine($"-----------------------------------");
    }

    private async Task Stress()
    {
        var start = DateTime.Now;
        Console.WriteLine("------------------------------------");
        Console.WriteLine("Inserting 100.000 records into the database...");
        Console.WriteLine(">    Starting...");
        
        var interation = 100001;
        var insertAsync = new List<Task>();
        for (int i = 0; i < interation; i++)
        {
            insertAsync.Add(InsertSong());

        }

        await Task.WhenAll(insertAsync);
        
        await Console.Out.WriteLineAsync($">    Time elapsed: {start.Second} seconds");
    }

    private async Task InsertSong()
    {
        var song = new Song()
        {
            Id = Guid.NewGuid(),
            Album = "Test Album",
            Artist = "Test Artist",
            Title = "Test Song"
        };

        await _dataBase.Add(song);
    }

    private Task Exit()
        =>throw new SystemException("See ya!");
}