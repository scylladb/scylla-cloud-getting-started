package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	internal "github.com/Canhassi12/scylla-cloud-getting-started/Internal/commands"
	"github.com/Canhassi12/scylla-cloud-getting-started/Internal/database"
	"github.com/joho/godotenv"
	"github.com/scylladb/gocqlx/v2"
)

func main() {
	godotenv.Load(".env")
	
	session, err := database.New().Connect(); if err != nil {
		println("error database connection =>", err.Error())
		return
	}

	err = executeMigrateFile(session, "./Internal/database/migrations/migrate.cql"); if err != nil {
		println(err.Error())
		return
	}

	reader := bufio.NewReader(os.Stdin)

	for {
		fmt.Print("\nAvailable commands:\n!add - add a new song\n!list - list all registered songs\n!delete - delete a specific song\n!stress - stress testing with mocked data\n!q - Quit the console\n\n")
		
		input, err := reader.ReadString('\n'); if err != nil {
			fmt.Println("Error to read the input:", err)
			break
		}

		command := strings.TrimSpace(input)

		switch command {
		case "!add":

			fmt.Println("Insert song title: ")
			titleInput, err := reader.ReadString('\n'); if err != nil {
				fmt.Println("Error to read the input:", err.Error())
				break
			}

			title := strings.TrimSpace(titleInput)

			fmt.Println("Insert album name: ")
			albumInput, err := reader.ReadString('\n'); if err != nil {
				fmt.Println("Error to read the input:", err.Error())
				break
			}

			album := strings.TrimSpace(albumInput)

			fmt.Println("Insert artist name:")
			artistInput, err := reader.ReadString('\n'); if err != nil {
				fmt.Println("Error to read the input:", err.Error())
				break
			}

			artist := strings.TrimSpace(artistInput)

			song := &database.Song {
				Title: title,
				Artist: artist,
				Album: album,
			}

			err = internal.NewAddSongCommand().Insert(session, song); if err != nil {
				println(err.Error())
				continue
			}
		
		case "!list":
			songs, err := internal.NewListSongCommand().List(session); if err != nil {
				println(err.Error())
				break
			}

			for _, song := range songs {
				fmt.Println(song)
			}
		case "!delete": 
			err := internal.NewDeleteSongCommand().Delete(session); if err != nil {
				println(err.Error())
			}

		case "!stress":
			err := internal.NewStressTestingCommand().Stress(session); if err != nil {
				println(err.Error())
			}
			
		case "!q": 
			fmt.Println("Exiting...")

			return
		default: 
			continue
		}
	}
}

func executeMigrateFile(session *gocqlx.Session, filePath string) error {
	content, err := os.ReadFile(filePath)
	if err != nil {
		return fmt.Errorf("error to read migrate file: %w", err)
	}

	queries := strings.Split(string(content), ";")

	for _, query := range queries {
		query = strings.TrimSpace(query)
		if query == "" {
			continue
		}
		
		q := session.Query(query, nil)

		if err := q.Exec(); err != nil {
			return fmt.Errorf("error to exec migrate query: %w", err)
		}
	}

	return nil
}
