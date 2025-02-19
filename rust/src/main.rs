mod commands;
mod songs;
mod migrate;
mod repository;

use clap::Parser;
use std::{io, sync::Arc};
use std::time::Duration;
use scylla::{Session, SessionBuilder};
use migrate::migrate_database;
use repository::SongRepository;

/// Simple program to greet a person
#[derive(Parser, Default, Debug)]
#[clap(author = "danielhe4rt", version, about)]
pub struct ConnectionDetails {
    /// Scylla Cloud Node URL's
    #[arg(num_args = 1..3, value_parser, value_delimiter = ',')]
    pub nodes: Vec<String>,

    /// Cluster Username
    #[arg(short, long)]
    username: String,

    /// Cluster Password
    #[arg(short, long)]
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = ConnectionDetails::parse();
    let database = db_connect(&args).await;
    migrate_database(&database).await?;
    let repository = Arc::new(SongRepository::new(database).await);

    println!("------------------------------------");
    println!("- ScyllaDB Cloud Rust Media Player -");
    println!("------------------------------------");
    println!("-    Leave a star on the repo      -");
    println!("-     https://bit.ly/scy-gh        -");
    println!("------------------------------------");

    println!("-----------------------------------");

    display_help();

    loop {
        let command = get_command();

        let _ = match command.as_str().trim() {
            "!add" => commands::add_song(&repository).await,
            "!list" => commands::list_songs(&repository).await,
            "!delete" => commands::delete_song(&repository).await,
            "!stress" => commands::stress(Arc::clone(&repository)).await,
            "!q" => panic!("See ya!"),
            _ => Ok(()),
        };
        display_help();
    }
}

fn get_command() -> String {
    println!("Type any command: ");
    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Something unexpected happened.");

    if command.is_empty() {
        display_help();
        return get_command();
    }

    return command;
}

fn display_help() -> () {
    println!("------------------------------------");
    println!("Here some possibilities");
    println!("  !add - add new song");
    println!("  !list - list all songs");
    println!("  !delete - delete a specific song");
    println!("  !stress - stress testing with mocked data");
    println!("------------------------------------");
}


async fn db_connect(config: &ConnectionDetails) -> Arc<Session> {
    let nodes = config
        .nodes
        .iter()
        .filter(|i| !i.is_empty())
        .collect::<Vec<_>>();

    Arc::new(SessionBuilder::new()
        .known_nodes(nodes)
        .connection_timeout(Duration::from_secs(5))
        .user(config.username.to_string(), config.password.to_string())
        .build()
        .await
        .expect("Connection Refused. Check your credentials and your IP linked on the ScyllaDB Cloud."))
}