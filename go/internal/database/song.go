package database

import (
	"fmt"
	"time"
)

type Song struct {
	Id string
	Title string
	Artist string
	Album string
	Created_at time.Time
}

func (s Song) String() string {
	return fmt.Sprintf("Id: %s\nTitle: %s\nArtist: %s\nAlbum: %s\nCreated At: %s\n", s.Id, s.Title, s.Artist, s.Album, s.Created_at)
}
