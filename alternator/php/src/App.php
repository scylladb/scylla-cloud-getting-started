<?php

namespace App;

use App\Commands\DeleteSongCommand;
use App\Commands\InsertSongCommand;
use App\Commands\ListSongsCommand;
use Aws\DynamoDb\DynamoDbClient;

class App
{
    const PREFIX = '!';

    public function start(): void
    {
        $alternatorClient = new DynamoDbClient([
            'endpoint' => 'http://localhost:8000',
            'credentials' => ['key' => 'None', 'secret' => 'None'],
            'region' => 'None'
        ]);

        Migration::run($alternatorClient);

        echo "ScyllaDB Alternator CLI" . PHP_EOL;

        while (true) {
            echo "Enter a command: " . PHP_EOL;
            $input = trim(fgets(STDIN));

            if (!str_starts_with($input, self::PREFIX)) {
                echo 'Commands available: !list-songs, !insert-song, !delete-song';
                continue;
            }

            $command = match ($input) {
                '!list-songs' => new ListSongsCommand(),
                '!insert-song' => new InsertSongCommand(),
                '!delete-song' => new DeleteSongCommand(),
            };

            $command->run($alternatorClient);
        }
    }
}