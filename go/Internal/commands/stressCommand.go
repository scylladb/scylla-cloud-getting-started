package internal

import (
	"fmt"
	"sync"
	"time"

	"github.com/scylladb/gocqlx/v2"
)


type StressTestingCommandInterface interface {
	Stress(*gocqlx.Session) error
}

type StressTestingCommand struct {

}

func NewStressTestingCommand() *StressTestingCommand {
	return &StressTestingCommand{}
}

func (command *StressTestingCommand) Stress(session *gocqlx.Session) error {
	fmt.Println("------------------------------------")
	fmt.Println("Inserting 100,000 records into the database...")
	fmt.Println("> Starting...")

	start := time.Now()

	var wg sync.WaitGroup
	sem := make(chan bool, 550)

	for i := 0; i < 100_000; i++ {
		sem <- true 

		wg.Add(1)
		go func() {
			defer func() {
				<-sem
				wg.Done()
			}()
			
			q := session.Query(`INSERT INTO media_player.playlist (id, title, artist, album, created_at) VALUES (now(), ?, ?, ?, ?)`,
			[]string{":title", ":artist", ":album", ":created_at"}).
			BindMap(map[string]interface{}{
				":title":      "title teste",
				":artist":     "artist teste",
				":album":      "album teste",
				":created_at": time.Now(),
			})

			err := q.Exec(); if err != nil {
				fmt.Println(err.Error())
			} 
		}()
	}

	wg.Wait()
	fmt.Println("Time taken:", time.Since(start))

	return nil
}
