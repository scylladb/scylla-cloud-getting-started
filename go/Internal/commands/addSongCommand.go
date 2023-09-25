package internal

import (
	"fmt"
	"time"

	"github.com/Canhassi12/scylla-cloud-getting-started/Internal/database"
	"github.com/scylladb/gocqlx/v2"
)


type AddSongCommandInterface interface {
	Insert(*gocqlx.Session, *database.Song) error
}

type AddSongCommand struct {

}

func NewAddSongCommand() (*AddSongCommand) {
	return &AddSongCommand{}
}

func (command *AddSongCommand) Insert(session *gocqlx.Session, song *database.Song) error {
	q := session.Query(
		`INSERT INTO media_player.playlist (id,title,artist,album,created_at) VALUES (now(),?,?,?,?)`,
		[]string{":title", ":artist", ":album", ":created_at"}).
		BindMap(map[string]interface{} {
			":title":      song.Title,
			":artist":     song.Artist,
			":album":      song.Album,
			":created_at": time.Now(),
		})

	err := q.Exec(); if err != nil {
		return fmt.Errorf("error in exec query to insert a song in playlist %w", err)
	}

	return nil
}
 