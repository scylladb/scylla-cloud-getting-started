<?php

namespace App\Song\DTOs;

use Ramsey\Uuid\Uuid;

class InsertSongDTO
{
    public function __construct(
        public readonly string $title,
        public readonly string $album,
        public readonly string $artist,
    )
    {
    }

    public static function make(string $title, string $album, string $artist): self
    {
        return new self($title, $album, $artist);
    }
}