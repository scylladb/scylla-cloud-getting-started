<?php

namespace App\Commands;

use App\Song\DTOs\DeleteSongDTO;
use App\Song\Song;
use Aws\DynamoDb\DynamoDbClient;
use Aws\DynamoDb\Marshaler;

class DeleteSongCommand extends AbstractCommand
{
    private readonly Marshaler $marshaler;

    public function __construct()
    {
        $this->marshaler = new Marshaler();
    }

    public function run(DynamoDbClient $client): CommandResponseEnum
    {

        $songs = $this->retrieveSongList($client);

        foreach ($songs as $key => $song) {
            echo $this->formatSong($key, $song);
        }

        $songIndex = (int)$this->input('Select the index of the desired song to be deleted: ');
        if (!array_key_exists($songIndex, $songs)) {
            throw new \Exception('Song index not found.');
        }

        $songToBeDeleted = $songs[$songIndex];

        $preparedQuery = $this->prepareDeleteQuery($songToBeDeleted);
        $client->deleteItem($preparedQuery);

        $this->info(sprintf('Song %s deleted.', $songToBeDeleted->id));

        return CommandResponseEnum::SUCCESS;
    }

    /**
     * @param DynamoDbClient $client
     * @return array<int, Song>
     */
    private function retrieveSongList(DynamoDbClient $client): array
    {
        $results = $client->scan($this->listSongsQuery());
        $songs = [];
        foreach ($results['Items'] as $item) {
            $parsedItem = $this->marshaler->unmarshalItem($item);
            $songs[] = Song::fromItem($parsedItem);
        }

        return $songs;
    }

    private function formatSong(int $key, Song $song): string
    {
        return sprintf(
            'Index: %s | Song: %s | Artist: %s | Album: %s | Created At: %s %s',
            $key,
            $song->title,
            $song->artist,
            $song->album,
            $song->createdAt->format('Y-m-d H:i:s'),
            PHP_EOL
        );
    }

    private function listSongsQuery(): array
    {
        return ['TableName' => 'songs'];
    }

    private function prepareDeleteQuery(Song $song): array
    {
        return [
            'TableName' => 'songs',
            'Key' => [
                'id' => ['S' => $song->id->toString()],
                'created_at' => ['S' => $song->createdAt->format('Y-m-d H:i:s')],
            ],
        ];
    }
}