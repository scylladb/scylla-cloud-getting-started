# Quick Start: Ruby

In this tutorial we're gonna build a simple Media Player to store our songs and build playlists

## 1. Setup the Environment

### 1.1 Downloading rust and dependencies:

If you don't have ruby installed already on your machine, you can install from two possible sources:

1. [Ruby main website](https://www.ruby-lang.org/en/downloads/)
2. [Rbenv](https://github.com/rbenv/rbenv)

After installing the language, make sure to install the `bundler` library so we can manage our projects with:

```sh
gem install bundler
```

### 1.2 Starting the project

Now with the ruby and bundler gem installed, let's create a new project with the following command:

```sh
mkdir media_player && cd media_player && bundle init
```

### 1.3 Setting the project dependencies

First we'll install the required gem to connect to scyllaDB with the following command:

```sh
bundle add cassandra-driver
```

This gem can be found at [github](https://github.com/datastax/ruby-driver/)

> Disclaimer: This gem require system wide dependencies with the cassandra client, so it's required to install on your system (or run the whole application under a docker image). You can find the installation guide at: https://cassandra.apache.org/doc/latest/cassandra/getting_started/installing.html

A sample `Gemfile` will be like this:

```ruby
# frozen_string_literal: true

source 'https://rubygems.org'

gem 'cassandra-driver', '~> 3.2'
```

## 2. Connecting to the Cluster

Make sure to get the right credentials on your [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) in the tab `Connect`.

```ruby
# frozen_string_literal: true

require 'cassandra'

cluster = Cassandra.cluster(
  username: 'scylla',
  password: 'a-very-secure-password',
  hosts: [
    'node-0.aws-sa-east-1.xxx.clusters.scylla.cloud',
    'node-1.aws-sa-east-1.xxx.clusters.scylla.cloud',
    'node-2.aws-sa-east-1.xxx.clusters.scylla.cloud'
  ]
)
```

> If the connection got refused, check if your IP Address is added into allowed IPs.

## 3. Handling Queries

Using the `cassandra` gem you can instantiate a session and then run fully asynchronous queries.

```ruby
session  = cluster.connect

future = session.execute_async('SELECT address, port, connection_stage FROM system.clients LIMIT 5')
future.on_success do |rows|
  rows.each do |row|
    puts "IP -> #{row[:address]}, Port -> #{row[:port]}, CS -> #{row[:connection_stage]}"
  end
end

future.join
```
