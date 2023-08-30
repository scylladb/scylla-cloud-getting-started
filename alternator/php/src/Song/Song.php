<?php

namespace App\Song;

use DateTimeInterface;
use Ramsey\Uuid\Uuid;
use Ramsey\Uuid\UuidInterface;

class Song
{
    public function __construct(
        public UuidInterface     $id,
        public string            $title,
        public string            $album,
        public string            $artist,
        public DateTimeInterface $createdAt
    )
    {
    }

    public static function fromItem(array $item): self
    {
        return new self(
            id: Uuid::fromString($item['id']),
            title: $item['title'],
            album: $item['album'],
            artist: $item['artist'],
            createdAt: \DateTimeImmutable::createFromFormat('Y-m-d H:i:s', $item['created_at'])
        );
    }
}