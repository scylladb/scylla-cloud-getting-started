package internal

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/Canhassi12/scylla-cloud-getting-started/Internal/database"
	"github.com/scylladb/gocqlx/v2"
)

type DeleteSongCommandInterface interface {
	Delete(*gocqlx.Session) error
}

type DeleteSongCommand struct {

}

func NewDeleteSongCommand() *DeleteSongCommand {
	return &DeleteSongCommand{}
}

func (cmd *DeleteSongCommand) Delete(session *gocqlx.Session) error {
	songs, err := NewListSongCommand().List(session); if err != nil {
		return err
	}

	index, err := cmd.selectSongToDelete(songs); if err != nil {
		return err
	}

	if index >= 0 && index < len(songs) {
		songToDelete := songs[index]

		q := session.Query(`DELETE FROM media_player.playlist WHERE id = ?`,
		[]string{":id"}).
		BindMap(map[string]interface{} {
			":id": songToDelete.Id,
		})

		err := q.Exec(); if err != nil {
			return fmt.Errorf("error to exec delete query %w", err)
		}
	}
		
	return nil
}

func (cmd *DeleteSongCommand) selectSongToDelete(songs []database.Song) (int, error) {
	for i, song := range songs {
		fmt.Printf("Index: %d  | Song: %s | Album: %s | Artist: %s | Created At: %s\n", i+1, song.Title, song.Album, song.Artist, song.Created_at)
	}

	fmt.Print("Select an index to be deleted: ")
	
	reader := bufio.NewReader(os.Stdin)

	input, err := reader.ReadString('\n'); if err != nil {
		return 0, fmt.Errorf("error to read the input %w", err)
	}

	idString := strings.TrimSpace(input)

	id, err := strconv.Atoi(idString); if err != nil {
		return id, fmt.Errorf("that index is invalid %w", err)
	}

	return id - 1, nil
}
 