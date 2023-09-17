using MediaPlayer;

namespace MediaPlayer;

public class Program
{
    static async Task Main(string[] args)
    {
        
        var cli = new Cli(args);
        await cli.Intro();

        await cli.Start();
    }
}

