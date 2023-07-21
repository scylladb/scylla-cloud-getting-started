use std::io;

use chrono::{DateTime, Local};

use crate::{database::Database, songs::Song};
use uuid::{self, Uuid};

pub fn add_song(database: &mut Database) -> () {
    let mut song = Song::default();

    println!("Type the song id: ");
    song.id = retrieve_input();

    println!("Type song title: ");
    song.title = retrieve_input();

    println!("Type the Artist name: ");
    song.artist = retrieve_input();

    let local: DateTime<Local> = Local::now();
    song.created_at = String::from(local.to_string());

    database.add(song);

    println!("Song Added!");
}

fn retrieve_input() -> String {
    let mut data = String::new();

    io::stdin()
        .read_line(&mut data)
        .expect("Something unexpected happened.");

    data
}

pub fn list_songs(database: &Database) {
    println!("Here is the songs added so far: ");
    println!("-----------------------------------");

    for (i, song) in database.songs.iter().enumerate() {
        println!("I: {} -> Song: {}", i, song.title.as_str().trim());
    }

    println!("-----------------------------------");
}

pub fn delete_song(database: &mut Database) {
    if database.songs.is_empty() {
        println!("No songs to be deleted yet.");
        return;
    }

    list_songs(&database);
    println!("Select a index to be deleting:");

    let option = retrieve_input()
        .trim()
        .parse::<usize>()
        .expect("Input failed");

    database.remove(option)
}

pub fn stress(database: &mut Database) {
    loop {
        let mut song: Song = Song::default();
        song.id = Uuid::new_v4().to_string();

        database.add(song)
    }
}
