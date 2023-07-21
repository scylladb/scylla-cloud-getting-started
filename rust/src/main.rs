mod commands;
mod database;
mod songs;

use database::Database;
use std::io;

fn main() {
    display_help();
    let mut database = Database::new();

    loop {
        let command = get_command();

        match command.as_str().trim() {
            "!add" => commands::add_song(&mut database),
            "!list" => commands::list_songs(&database),
            "!delete" => commands::delete_song(&mut database),
            "!stress" => commands::stress(&mut database),
            "!q" => panic!("cya"),
            _ => {}
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
