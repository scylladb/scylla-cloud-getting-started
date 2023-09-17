using MediaPlayer.Helper;
using MediaPlayer.Models;

namespace MediaPlayer;

public class Cli
{
    private readonly DataBase _dataBase;

    public Cli(string[] args)
    {
        var isConnectionValid = ValidateArguments(args);
        if(isConnectionValid)
            _dataBase = new DataBase(args);
        else
            throw new SystemException("See ya!");
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
                switch (command)
                {
                    case "!add":
                        await AddSong();
                        break;
                    case "!list":
                        await ListSongs();
                        break;
                    case "!delete":
                        await DeleteSong();
                        break;
                    case "!stress":
                        await Stress();
                        break;
                    case "!q":
                        await Exit();
                        break;
                    default:
                        await Start();
                        break;
                }
                DisplayHint();
            }
        }
        catch(Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
    }

    private string GetCommand()
    {
        Console.Write("Type any command: ");
        var command = Console.ReadLine();

        Console.WriteLine("");
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

        ShowSongList(songs);
        Console.WriteLine($"-----------------------------------");
        Console.WriteLine("");
    }

    private async Task Stress()
    {
        var start = DateTime.Now;
        Console.WriteLine("------------------------------------");
        Console.WriteLine("Inserting 100.000 records into the database...");
        Console.WriteLine(">    Starting...");
        
        var interation = 10000;
        var insertAsync = new List<Task>();
        for (int i = 0; i < interation; i++)
        {
            insertAsync.Add(InsertSong());
        }

        await Task.WhenAll(insertAsync);

        var timeElapsedToSecond = DateTime.Now - start;
        await Console.Out.WriteLineAsync($">    Time elapsed: {timeElapsedToSecond.Seconds} seconds");
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

    private async Task DeleteSong()
    {
        var songs = await _dataBase.ListSongs();
        ShowSongList(songs, true);
        Console.Write("Select a index to be deleted: ");
        var indexSelected = Console.ReadLine();
        var song = songs[Convert.ToInt32(indexSelected) - 1];
        Console.WriteLine($"Song {song.Title} from artist {song.Artist} Deleted!");
        await _dataBase.Delete(song);

    }
    
    private bool ValidateArguments(string[] args)
    {
        return args.Length switch
        {
            0 => ValidationHelper.ArgsValidation("Missing all arguments."),
            1 => ValidationHelper.ArgsValidation("Missing password credential."),
            2 => ValidationHelper.ArgsValidation("0 of 3 nodes were provided."),
            3 => ValidationHelper.ArgsValidation("1 of 3 nodes were provided."),
            4 => ValidationHelper.ArgsValidation("2 of 3 nodes were provided."),
            5 => true,
            _ => ValidationHelper.ArgsValidation("Too many arguments")
        };
    }

    private void ShowSongList(List<Song> songs, bool withIndex = false)
    {
        foreach (var (song, index) in songs.Select((v, i)=>(v, i)))
        {
            Console.WriteLine($"{(withIndex ? $"Index: {index + 1}" : $"ID: {song.Id}")} | Song: {song.Title} | Album: {song.Album} | Artist: {song.Artist} | Created At: {song.CreatedAt}");
        }
    }

    private Task Exit()
        => throw new SystemException("See ya!");
}