package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/Canhassi12/scylla-cloud-getting-started/internal/controllers"
	"github.com/Canhassi12/scylla-cloud-getting-started/internal/database"
	"github.com/joho/godotenv"
	"github.com/scylladb/gocqlx/v2"
)

func main() {
	godotenv.Load(".env")
	
	session, err := database.Connect(); 
	if err != nil {
		println("error database connection =>", err.Error())
		return
	}

	migratePath := os.Getenv("MIGRATE_PATH")

	if err := executeMigrateFile(session, migratePath); err != nil {
		println(err.Error())
		os.Exit(1)
	}

	reader := bufio.NewReader(os.Stdin)

	songController := controllers.NewSongController(session)
	stressController := controllers.NewStressController(session)

	for {
		fmt.Print("\nAvailable commands:\n!add - add a new song\n!list - list all registered songs\n!delete - delete a specific song\n!stress - stress testing with mocked data\n!q - Quit the console\n\n")
		
		input, _ := reader.ReadString('\n')
		command := strings.TrimSpace(input)

		switch command {
		case "!add":
			fmt.Println("Insert song title: ")
			titleInput, _ := reader.ReadString('\n')
			title := strings.TrimSpace(titleInput)

			fmt.Println("Insert album name: ")
			albumInput, _ := reader.ReadString('\n')
			album := strings.TrimSpace(albumInput)

			fmt.Println("Insert artist name:")
			artistInput, _ := reader.ReadString('\n')
			artist := strings.TrimSpace(artistInput)

			song := &database.Song {
				Title: title,
				Artist: artist,
				Album: album,
			}

			if err = songController.Insert(song); err != nil {
				println(err.Error())
			}
		
		case "!list":
			songs, err := songController.List()
			if err != nil {
				println(err.Error())
				break
			}

			for _, song := range songs {
				fmt.Println(song)
			}
			
		case "!delete": 
			if err := songController.Delete(); err != nil {
				println(err.Error())
			}

		case "!stress":
			if err := stressController.Stress(); err != nil {
				println(err.Error())
			}
			
		case "!q": 
			fmt.Println("Exiting...")
			os.Exit(0)

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
		query = strings.Trim(query, " \r\n")
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
