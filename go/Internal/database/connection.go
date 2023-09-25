package database

import (
	"os"
	"strings"

	"github.com/gocql/gocql"
	"github.com/scylladb/gocqlx/v2"
)

type DbInterface interface {
	Connect() (*gocqlx.Session, error)
}

type Db struct {

}

func New() *Db {
	return &Db{}
}

func (db *Db) Connect() (*gocqlx.Session, error) {
	nodes := os.Getenv("NODES")
	username := os.Getenv("CLUSTER_USERNAME")
	password := os.Getenv("CLUSTER_PASSWORD")
	region :=  os.Getenv("CLUSTER_REGION")

	hosts := strings.Split(nodes, ",")

	cluster := gocql.NewCluster(hosts...)

	cluster.Authenticator = gocql.PasswordAuthenticator{Username: username, Password: password}
	cluster.PoolConfig.HostSelectionPolicy = gocql.DCAwareRoundRobinPolicy(region)

	session, err := gocqlx.WrapSession(cluster.CreateSession())

	if err != nil {
		return nil, err
	}

	return &session, nil
}
