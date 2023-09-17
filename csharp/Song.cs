namespace MediaPlayer;

public record Song
{
    public Guid Id { get; init; }
    public string Title { get; init; }
    public string Album { get; init; }
    public string Artist { get; init; }
    public DateTime CreatedAt { get; init; } = DateTime.Now;
}