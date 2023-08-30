<?php

namespace App\Commands;

use App\Song\DTOs\InsertSongDTO;
use Aws\DynamoDb\DynamoDbClient;
use Aws\DynamoDb\Marshaler;
use Ramsey\Uuid\Uuid;

class InsertSongCommand extends AbstractCommand
{

    public function run(DynamoDbClient $client): CommandResponseEnum
    {
        $songTitle = $this->input('What is the song title that you want to insert?');
        $artistName = $this->input('What is the artist name of this song?');
        $albumName = $this->input('What is the album name of this song?');

        $songDTO = InsertSongDTO::make($songTitle, $albumName, $artistName);
        $preparedQuery = $this->preparedInsertQuery($songDTO);

        $client->putItem($preparedQuery);

        $this->info(sprintf('Song "%s" added.', $songDTO->title));

        return CommandResponseEnum::SUCCESS;
    }

    private function preparedInsertQuery(InsertSongDTO $songDTO): array
    {
        return [
            'TableName' => 'songs',
            'Item' => [
                'id' => ['S' => Uuid::uuid4()->toString()],
                'created_at' => ['S' => date('Y-m-d H:i:s')],
                'title' => ['S' => $songDTO->title],
                'album' => ['S' => $songDTO->album],
                'artist' => ['S' => $songDTO->artist],
            ]
        ];
    }
}