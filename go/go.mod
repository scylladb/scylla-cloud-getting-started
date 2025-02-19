module github.com/Canhassi12/scylla-cloud-getting-started

go 1.20

require (
	github.com/gocql/gocql v1.7.0
	github.com/joho/godotenv v1.5.1
	github.com/scylladb/gocqlx/v2 v2.8.0
)

require (
	github.com/hailocab/go-hostpool v0.0.0-20160125115350-e80d13ce29ed // indirect
	github.com/klauspost/compress v1.17.9 // indirect
	github.com/scylladb/go-reflectx v1.0.1 // indirect
	gopkg.in/inf.v0 v0.9.1 // indirect
)

replace github.com/gocql/gocql => github.com/scylladb/gocql v1.14.5
