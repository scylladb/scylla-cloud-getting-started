<?php
namespace App\Song\DTOs;

use DateTimeInterface;
use Ramsey\Uuid\Uuid;

class DeleteSongDTO
{
    public function __construct(
        public readonly Uuid $uuid,
        public readonly DateTimeInterface $createdAt
    )
    {
    }
}