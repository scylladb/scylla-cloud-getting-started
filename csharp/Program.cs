namespace MediaPlayer;

public class Program
{
    static async Task Main(string[] args)
    {
        try
        {
            var cli = new Cli(args);
            await cli.Intro();

            await cli.Start();
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);  
        }
    }
}

