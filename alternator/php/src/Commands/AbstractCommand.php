<?php

namespace App\Commands;

use Aws\DynamoDb\DynamoDbClient;

abstract class AbstractCommand
{
    abstract public function run(DynamoDbClient $client): CommandResponseEnum;

    /**
     * Returns the trimmed User Input
     * @param string $question
     * @return string
     */
    protected function input(string $question = ''): string
    {
        if (!empty($question)) {
            $this->info($question);
        }
        echo "> ";
        return trim(fgets(STDIN));
    }

    protected function info(string $message): void
    {
        echo sprintf('-> %s %s', $message, PHP_EOL);
    }
}