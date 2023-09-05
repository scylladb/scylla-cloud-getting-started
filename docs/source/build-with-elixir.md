# Quick Start: Elixir

## 1. Setup the Environment

### 1.1 Downloading Elixir and dependencies:

If you don't have Elixir and Erlang installed already on your machine, you can install from two possible sources:

1. [Elixir main website](https://elixir-lang.org/install.html)
2. [asdf](https://asdf-vm.com/guide/getting-started.html)
> NOTE: After installing asdf correctly make sure to install [Erlang](https://github.com/asdf-vm/asdf-erlang) first and then [Elixir](https://github.com/asdf-vm/asdf-elixir).

### 1.2 Starting the project

Now with Elixir properly installed you can create a project using [mix](https://elixir-lang.org/getting-started/mix-otp/introduction-to-mix.html). To create our project just run:

```sh
mix new media_player
```

### 1.3 Setting the project dependencies

Let's do a quick change into our `mix.exs` and add our project dependencies.

```exs
  defp deps do
    [
      {:decimal, "~> 1.0"},
      {:xandra, "~> 0.14"}
    ]
  end
```

- [Decimal](https://hexdocs.pm/decimal/readme.html): Arbitrary precision decimal arithmetic
- [Xandra](https://github.com/lexhide/xandra): Fast, simple, and robust Cassandra/ScyllaDB driver for Elixir
> NOTE: In this tutorial we are going to prepare an interactive project so that you can perform tests using Elixir's interactive shell.

## 2. Connecting to the Cluster

Make sure to get the right credentials on your [ScyllaDB Cloud Dashboard](https://cloud.scylladb.com/clusters) in the tab `Connect`.

```ex
options = [username: "scylla", password: "a-very-secure-password"]

{:ok, cluster} =
  Xandra.Cluster.start_link(
    authentication: {Xandra.Authenticator.Password, options},
    nodes: [
      "node-0.aws-sa-east-1.xxx.clusters.scylla.cloud",
      "node-1.aws-sa-east-1.xxx.clusters.scylla.cloud",
      "node-2.aws-sa-east-1.xxx.clusters.scylla.cloud"
    ],
    pool_size: 10
  )
```

> If the connection got refused, check if your IP Address is added into allowed IPs.