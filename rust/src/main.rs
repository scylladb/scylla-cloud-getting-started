mod commands;
mod database;
mod songs;
mod datetime;

use database::Database;
use std::{io, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    display_help();
    let mut database = Database::new().await;

    loop {
        let command = get_command();

        let _ = match command.as_str().trim() {
            "!add" => commands::add_song(&mut database).await,
            "!list" => commands::list_songs(&database).await,
            "!delete" => commands::delete_song(&mut database).await,
            "!stress" => commands::stress(Arc::new(Database::new().await)).await,
            "!q" => panic!("cya"),
            _ => Ok(())
        };
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
    println!("-----------");
    println!("Here some possibilities");
    println!("  !add - add new song");
    println!("  !list - list songs");
    println!("-----------");
}
