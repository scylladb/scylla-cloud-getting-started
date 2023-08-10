mod commands;
mod database;
mod datetime;
mod songs;
mod migrate;

use clap::Parser;
use database::Database;
use std::{io, sync::Arc};
use migrate::migrate_database;

/// Simple program to greet a person
#[derive(Parser, Default, Debug)]
#[clap(author = "danielhe4rt", version, about)]
pub struct ConnectionDetails {
    /// Scylla Cloud Node URL's
    #[arg(num_args = 3, value_parser, value_delimiter = ',')]
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
    let mut database = Database::new(&args).await;

    println!("------------------------------------");
    println!("- ScyllaDB Cloud Rust Media Player -");
    println!("------------------------------------");
    println!("-    Leave a star on the repo      -");
    println!("-     https://bit.ly/scy-gh        -");
    println!("------------------------------------");
    migrate_database(&database).await?;
    println!("-----------------------------------");
    
    display_help();

    loop {
        let command = get_command();

        let _ = match command.as_str().trim() {
            "!add" => commands::add_song(&mut database).await,
            "!list" => commands::list_songs(&database).await,
            "!delete" => commands::delete_song(&mut database).await,
            "!stress" => commands::stress(Arc::new(Database::new(&args).await)).await,
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
