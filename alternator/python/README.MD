# ScyllaDB Cloud Media Player Metrics

Project to store songs that you like to listen daily and keep track of them using the command line.

## Prerequisites

* [Python 3.7+](https://www.python.org/downloads/)
* [Virtualenv](https://virtualenv.pypa.io/en/latest/installation.html)

## Running the project

Clone the repository:

```sh 
git clone https://github.com/scylladb/scylla-cloud-getting-started.git
cd scylla-cloud-getting-started/alternator/python
```

Start a new virtual environment for the project:

```sh
virtualenv env
source env/bin/activate
```

Install the driver: 

```sh
pip install boto3
```

Go to `environment.py` and set up your ScyllaDB Cloud Cluster credentials and run:

```sh
python cli.py
```

## Available Commands

| Command  | Description |
|---|---|
| !new   | Add a new song to your liked songs list   |
| !delete  | Delete a specific song from your liked songs list   |
| !listen  | Creates a register of which song and when you listened to it  |
