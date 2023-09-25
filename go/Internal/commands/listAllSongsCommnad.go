package internal

import (
	"fmt"

	"github.com/Canhassi12/scylla-cloud-getting-started/Internal/database"
	"github.com/scylladb/gocqlx/v2"
)

type ListSongCommandInterface interface {
	List(*gocqlx.Session)
}

type ListSongCommand struct {

}

func NewListSongCommand() *ListSongCommand {
	return &ListSongCommand{}
}

func (command *ListSongCommand) List(session *gocqlx.Session) ([]database.Song, error) {
	song := []database.Song{}

	q := session.Query("SELECT * FROM media_player.playlist", nil);

	if err := q.SelectRelease(&song); err != nil {
		return song, fmt.Errorf("error in exec query to list playlists: %w", err)
	}

	return song, nil
}
 