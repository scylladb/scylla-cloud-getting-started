package controllers

import (
	"fmt"
	"sync"
	"time"

	"github.com/scylladb/gocqlx/v2"
)

type StressController struct {
	Session *gocqlx.Session
}

func NewStressController(session *gocqlx.Session) *StressController {
	return &StressController{Session: session}
}

func (c *StressController) Stress() error {
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
			
			q := c.Session.Query(`INSERT INTO media_player.playlist (id, title, artist, album, created_at) VALUES (now(), ?, ?, ?, ?)`,
			[]string{":title", ":artist", ":album", ":created_at"}).
			BindMap(map[string]interface{}{
				":title":      "title teste",
				":artist":     "artist teste",
				":album":      "album teste",
				":created_at": time.Now(),
			})

			if err := q.Exec(); err != nil {
				fmt.Println(err.Error())
			} 
		}()
	}

	wg.Wait()
	fmt.Println("Time taken:", time.Since(start))

	return nil
}
