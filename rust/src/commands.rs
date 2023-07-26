use std::sync::Arc;

use anyhow::anyhow;
use tokio::{io::{AsyncBufReadExt, BufReader}, task::JoinSet};

use crate::{database::Database, datetime::DateTime, songs::Song};
use uuid::{self, Uuid};

pub async fn add_song(database: &mut Database) -> Result<(), anyhow::Error> {
    let now = DateTime::now();

    let mut lines = BufReader::new(tokio::io::stdin()).lines();

    let song = Song {
        id: Uuid::new_v4(),
        title: lines
            .next_line()
            .await?
            .ok_or_else(|| "".to_owned())
            .unwrap(),
        album: lines
            .next_line()
            .await?
            .ok_or_else(|| "".to_owned())
            .unwrap(),
        artist: lines
            .next_line()
            .await?
            .ok_or_else(|| "".to_owned())
            .unwrap(),
        created_at: now,
    };

    database.add(song).await?;

    println!("Song Added!");
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
        .for_each(|row| println!("Read a value from row: {:?}", row.id));

    println!("-----------------------------------");

    Ok(())
}

pub async fn delete_song(database: &mut Database) -> Result<(), anyhow::Error> {
    if !database.songs.is_empty() {
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
    } else {
        Err(anyhow!("No songs found"))
    }
}

pub async fn stress(database: Arc<Database>) -> Result<(), anyhow::Error> {
    let start = std::time::Instant::now();
    let mut set = JoinSet::new();

    (1..100000).into_iter().for_each(|_| {
        let db = Arc::clone(&database);

        set.spawn(async move {
            db.add(Song {
                id: Uuid::new_v4(),
                title: String::from("lalala"),
                album: String::from("lalala"),
                artist: String::from("lalala"),
                created_at: DateTime::now(),
            })
            .await
        });
    });

    while let Some(res) = set.join_next().await {
        res?.unwrap();
    }

    println!("Time elapsed: {:?}", start.elapsed());

    Ok(())

}
