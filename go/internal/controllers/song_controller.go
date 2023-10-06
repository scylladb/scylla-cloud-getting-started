package controllers

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"

	"github.com/Canhassi12/scylla-cloud-getting-started/internal/database"
	"github.com/scylladb/gocqlx/v2"
)

type SongController struct {
    Session *gocqlx.Session
}

func NewSongController(db *gocqlx.Session) *SongController {
    return &SongController{Session: db}
}

func (c *SongController) Insert(song *database.Song) error {
	q := c.Session.Query(
		`INSERT INTO media_player.playlist (id,title,artist,album,created_at) VALUES (now(),?,?,?,?)`,
		[]string{":title", ":artist", ":album", ":created_at"}).
		BindMap(map[string]interface{} {
			":title":      song.Title,
			":artist":     song.Artist,
			":album":      song.Album,
			":created_at": time.Now(),
		})

	if err := q.Exec(); err != nil {
		return fmt.Errorf("error in exec query to insert a song in playlist %w", err)
	}

	return nil
}

func (c *SongController) List() ([]database.Song, error) {
	song := []database.Song{}

	q := c.Session.Query("SELECT * FROM media_player.playlist", nil);

	if err := q.SelectRelease(&song); err != nil {
		return song, fmt.Errorf("error in exec query to list playlists: %w", err)
	}

	return song, nil
}

func (c *SongController) Delete() error {
	songs, err := c.List()
	if err != nil {
		return err
	}

	index, err := c.selectSongToDelete(songs) 
	if err != nil {
		return err
	}

	if index >= 0 && index < len(songs) {
		songToDelete := songs[index]

		q := c.Session.Query(`DELETE FROM media_player.playlist WHERE id = ?`,
		[]string{":id"}).
		BindMap(map[string]interface{} {
			":id": songToDelete.Id,
		})

		if err := q.Exec(); err != nil {
			return fmt.Errorf("error to exec delete query %w", err)
		}
	}
		
	return nil
}

func (c *SongController) selectSongToDelete(songs []database.Song) (int, error) {
	for i, song := range songs {
		fmt.Printf("Index: %d  | Song: %s | Album: %s | Artist: %s | Created At: %s\n", i+1, song.Title, song.Album, song.Artist, song.Created_at)
	}

	fmt.Print("Select an index to be deleted: ")
	
	reader := bufio.NewReader(os.Stdin)

	input, _ := reader.ReadString('\n')
	idString := strings.TrimSpace(input)

	id, err := strconv.Atoi(idString)
	if err != nil {
		return id, fmt.Errorf("that index is invalid %w", err)
	}

	return id - 1, nil
}
