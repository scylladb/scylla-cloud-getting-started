<?php

namespace App\Commands;

use App\Song\Song;
use Aws\DynamoDb\DynamoDbClient;
use Aws\DynamoDb\Marshaler;

class ListSongsCommand extends AbstractCommand
{

    private readonly Marshaler $marshaler;

    public function __construct()
    {
        $this->marshaler = new Marshaler();
    }

    public function run(DynamoDbClient $client): CommandResponseEnum
    {
        $this->info('Listing the songs available on the database...');

        $results = $client->scan($this->listSongsQuery());


        foreach ($results['Items'] as $item) {
            $parsedItem = $this->marshaler->unmarshalItem($item);
            echo $this->formatSong(Song::fromItem($parsedItem));
        }

        return CommandResponseEnum::SUCCESS;
    }

    private function formatSong(Song $song): string
    {
        return sprintf(
            'ID: %s | Song: %s | Artist: %s | Album: %s | Created At: %s %s',
            $song->id->toString(),
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
}