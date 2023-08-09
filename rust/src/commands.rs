use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    task::JoinSet,
};

use crate::{database::Database, datetime::DateTime, songs::Song};
use uuid::{self, Uuid};

pub async fn add_song(database: &mut Database) -> Result<(), anyhow::Error> {
    let now = DateTime::now();

    let mut lines = BufReader::new(tokio::io::stdin()).lines();

    println!("Song name: ");

    let title = lines
        .next_line()
        .await?
        .ok_or_else(|| "".to_owned())
        .unwrap();

    println!("Album: ");

    let album = lines
        .next_line()
        .await?
        .ok_or_else(|| "".to_owned())
        .unwrap();

    println!("Artist: ");
    let artist = lines
        .next_line()
        .await?
        .ok_or_else(|| "".to_owned())
        .unwrap();

    let song = Song {
        id: Uuid::new_v4(),
        title,
        album,
        artist,
        created_at: now,
    };

    println!("Song '{}' from artist '{}' Added!", song.title, song.artist);

    database.add(song).await?;

    Ok(())
}

pub async fn list_songs(database: &Database) -> Result<(), anyhow::Error> {
    println!("Here is the songs added so far: ");
    println!("-----------------------------------");

    database
        .list()
        .await?
        .ok_or_else(|| Vec::<Song>::new())
        .unwrap()
        .into_iter()
        .for_each(|row| {
            println!(
                "ID: {} | Song: {} | Album: {} | Created At: {}",
                row.id,
                row.title,
                row.album,
                row.created_at.as_ref().to_string()
            )
        });

    println!("-----------------------------------");

    Ok(())
}

pub async fn delete_song(database: &mut Database) -> Result<(), anyhow::Error> {
    list_songs(&database).await?;
    println!("Select a index to be deleting:");
    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    let option = lines
        .next_line()
        .await?
        .ok_or_else(|| "".to_owned())
        .unwrap()
        .parse::<usize>()?;

    database.remove(option);

    Ok(())
}

pub async fn stress(database: Arc<Database>) -> Result<(), anyhow::Error> {
    println!("------------------------------------");
    println!("Inserting 100.000 records into the database...");
    println!(">    Starting...");

    let start = std::time::Instant::now();
    let mut set = JoinSet::new();

    (1..100000).into_iter().for_each(|_| {
        let db = Arc::clone(&database);

        set.spawn(async move {
            db.add(Song {
                id: Uuid::new_v4(),
                title: String::from("Test Song"),
                album: String::from("Test Title"),
                artist: String::from("Test Artist"),
                created_at: DateTime::now(),
            })
            .await
        });
    });

    while let Some(res) = set.join_next().await {
        res?.unwrap();
    }

    println!(">    Time elapsed: {} seconds", start.elapsed().as_secs());

    Ok(())
}
